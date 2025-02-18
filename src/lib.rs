// Import necessary traits and types from external crates
use mobc::async_trait;
use mobc::Manager;
use std::sync::Arc;
use surrealdb::{Surreal, Error};
use surrealdb::engine::any; // Enables runtime selection of engine

/// Enum representing the supported connection protocols.
#[derive(Debug, Clone)]
pub enum ConnectionProtocol {
    Http,
    Https,
    Ws,
    Wss,
}

impl ConnectionProtocol {
    /// Returns the scheme as a static string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionProtocol::Http => "http://",
            ConnectionProtocol::Https => "https://",
            ConnectionProtocol::Ws => "ws://",
            ConnectionProtocol::Wss => "wss://",
        }
    }
}

/// A high‑performance SurrealDB connection manager using static string slices.
/// The default connection protocol is WebSocket (ws), but users can override it.
pub struct SurrealDBConnectionManager {
    protocol: ConnectionProtocol, // The connection protocol; default is Ws.
    db_url: &'static str,         // Server address (host:port/path)
    db_user: &'static str,        // Username for authentication
    db_password: &'static str,    // Password for authentication
}

impl SurrealDBConnectionManager {
    /// Creates a new connection manager with the default protocol (ws).
    pub fn new(
        db_url: &'static str,
        db_user: &'static str,
        db_password: &'static str,
    ) -> Self {
        Self {
            protocol: ConnectionProtocol::Ws, // Default to ws
            db_url,
            db_user,
            db_password,
        }
    }

    /// Creates a new connection manager with a custom protocol.
    pub fn new_with_protocol(
        protocol: ConnectionProtocol,
        db_url: &'static str,
        db_user: &'static str,
        db_password: &'static str,
    ) -> Self {
        Self {
            protocol,
            db_url,
            db_user,
            db_password,
        }
    }
}

#[async_trait]
impl Manager for SurrealDBConnectionManager {
    // Use Surreal with the 'any' engine for runtime flexibility.
    type Connection = Arc<Surreal<any::Any>>;
    type Error = Error;

    /// Establish a new connection.
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // Construct the full URL by concatenating the protocol and the server address.
        let full_url = format!("{}{}", self.protocol.as_str(), self.db_url);
        let db = any::connect(full_url).await?;
        // Authenticate using the provided credentials.
        db.signin(surrealdb::opt::auth::Root {
            username: self.db_user,
            password: self.db_password,
        })
        .await?;
        // Return the connection wrapped in an Arc.
        Ok(Arc::new(db))
    }

    /// Check the health of an existing connection.
    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let mut response = conn.query("RETURN 1").await?;
        let result: Option<i32> = response.take(0)?;
        if result == Some(1) {
            Ok(conn)
        } else {
            Err(Error::Api(surrealdb::error::Api::Query(
                "Health check failed".into(),
            )))
        }
    }
}
