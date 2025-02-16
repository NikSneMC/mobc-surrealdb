// Import necessary traits and types from external crates
use mobc::async_trait; // For asynchronous trait support in mobc
use mobc::Manager; // The Manager trait for connection pooling
use surrealdb::engine::remote::ws::{Client, Ws}; // WebSocket client for SurrealDB
use surrealdb::{Surreal, Error}; // SurrealDB main struct and error type
use std::sync::Arc; // For thread-safe reference counting

// Define a struct to manage SurrealDB connection parameters
pub struct SurrealDBConnectionManager {
    db_url: String, // URL of the SurrealDB server
    db_user: String, // Username for authentication
    db_password: String, // Password for authentication
}

impl SurrealDBConnectionManager {
    // Constructor to create a new instance of the connection manager
    pub fn new(db_url: String, db_user: String, db_password: String) -> Self {
        Self {
            db_url,
            db_user,
            db_password,
        }
    }
}

// Implement the Manager trait for SurrealDBConnectionManager
#[async_trait]
impl Manager for SurrealDBConnectionManager {
    // Define the associated types for the connection and error
    type Connection = Arc<Surreal<Client>>; // Thread-safe reference to SurrealDB client
    type Error = Error; // Error type from SurrealDB

    // Asynchronously establish a new connection to the database
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // Create a new SurrealDB client using WebSocket
        let db = Surreal::new::<Ws>(&self.db_url).await?;
        // Authenticate with the provided username and password
        db.signin(surrealdb::opt::auth::Root {
            username: &self.db_user,
            password: &self.db_password,
        })
        .await?;
        // Return the client wrapped in an Arc for shared ownership
        Ok(Arc::new(db))
    }

    // Asynchronously check the health of an existing connection
    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        // Perform a simple query to verify the connection is alive
        let mut response = conn.query("RETURN 1").await?;
        let result: Option<i32> = response.take(0)?;
        // If the query returns 1, the connection is healthy
        if result == Some(1) {
            Ok(conn)
        } else {
            // Otherwise, return an error indicating the health check failed
            Err(Error::Api(surrealdb::error::Api::Query(
                "Health check failed".into(),
            )))
        }
    }
}
