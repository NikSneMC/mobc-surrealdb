# mobc-surrealdb

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)&nbsp;&nbsp;&nbsp;&nbsp;[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)&nbsp;&nbsp;&nbsp;&nbsp;[![crates.io](https://img.shields.io/badge/crates.io-latest-%23dea584)](https://crates.io/crates/mobc-surrealdb)

[mobc-surrealdb](https://crates.io/crates/mobc-surrealdb) is an asynchronous connection pool manager for [Surrealdb](https://surrealdb.com), that is built on top of the [mobc](https://crates.io/crates/mobc) library. Its design emphasizes high performance and flexibility while providing support for multiple connection protocols.

## Features

- **Asynchronous Connection Pooling:**  
  Leverages mobc to manage a pool of connections, reducing the overhead of establishing new connections.

- **Automatic Connection Recycling:**  
  Validates and reuses connections, ensuring optimal performance even under high load.

- **Customizable Pool Settings:**  
  Configure parameters such as maximum open connections, idle connections, and connection lifetime to suit your application's needs.

- **Protocol Flexibility:**  
  Supports multiple connection protocols (HTTP, HTTPS, WS, WSS). By default, the connection protocol is set to WebSocket (ws) for high performance, but you can override it using a custom constructor.

## Installation

Add `mobc-surrealdb` to your project's `Cargo.toml`:

```toml
[dependencies]
mobc-surrealdb = "0.2.0"
```
Alternatively, you can add it directly using the `cargo add` command:
```bash
cargo add mobc-surrealdb
```

## Dependencies
This crate depends on the following libraries:

- `mobc:` A generic connection pool for asynchronous Rust.

- `surrealdb:` The official SurrealDB client for Rust.

## Examples 
Here’s a quick example of how to use `mobc-surrealdb` in your project:
``` rust
// Import necessary crates and modules
use mobc::Pool; // The mobc connection pool
use mobc_surrealdb::SurrealDBConnectionManager; // The connection manager
use serde::{Deserialize, Serialize}; // For serializing/deserializing our data types
use std::time::Duration; // For configuring pool settings
use surrealdb::sql::Thing; // SurrealDB's type for record IDs
use tokio; // Tokio runtime for asynchronous execution

// Define a struct to represent a person, with fields for ID, name, and age.
// The ID is optional because it will be assigned by the database upon insertion.
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: Option<Thing>, // Optional record ID (assigned by the database)
    name: String,      // Person's name
    age: i64,          // Person's age (64-bit integer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the connection manager without specifying a protocol.
    // By default, it will use WebSocket (ws) as the connection protocol.
    let manager = SurrealDBConnectionManager::new(
        "127.0.0.1:8000", // SurrealDB server address
        "root",           // SurrealDB Default Username
        "root",           // SurrealDB Default Password
    );

    // Build the connection pool with custom settings:
    // - Maximum of 20 open connections.
    // - Maximum of 5 idle connections.
    // - Each connection lives for up to 300 seconds.
    // You can keep connections open indefinitely by using ".max_lifetime(None)".
    let pool = Pool::builder()
        .max_open(20)
        .max_idle(5)
        .max_lifetime(Some(Duration::from_secs(300))) 
        .build(manager);

    // Get a connection from the pool asynchronously.
    let conn = pool.get().await?;

    // Set the operational context by specifying the namespace and database.
    // SurrealDB requires a namespace and database context to execute queries.
    conn.use_ns("accounts").use_db("users").await?;

    // Create a new person instance.
    // The ID is None because it will be auto-assigned by the database upon insertion.
    let person = Person {
        id: None,
        name: "Alice".to_string(),
        age: 30,
    };

    // Insert the person into the "user" table.
    let _created: Option<Person> = conn.create("user").content(person).await?;

    // Retrieve all records from the "user" table with a SQL query.
    let mut response = conn.query("SELECT * FROM user").await?;
    let persons: Vec<Person> = response.take(0)?;

    // Iterate over each person and print their details.
    for person in persons {
        println!("******************");
        // Check if the person has an assigned ID.
        if let Some(ref thing) = person.id {
            // If the ID is a string variant, print it.
            if let surrealdb::sql::Id::String(ref id_str) = thing.id {
                println!("id: {}", id_str);
            } else {
                println!("id: Non-string ID");
            }
        } else {
            println!("id: None");
        }
        println!("name: {}", person.name);
        println!("age: {}", person.age);
        println!("******************");
    }

    Ok(())
}


```
If you would like to explicitly define a connection protocol, you can make use of the `new_with_protocol` constructor instead of the default `new` constructor. This approach allows you to specify the connection protocol, offering greater flexibility in how you connect to SurrealDB. Below is an example demonstrating how to utilize this function effectively for your use case:
``` rust
    // Import both connection manager and protocol enum
    use mobc_surrealdb::{ConnectionProtocol , SurrealDBConnectionManager};

    // Initialize the connection manager using a custom protocol.
    // Here, we choose HTTP as the connection protocol by using the `new_with_protocol` constructor.
    let manager = SurrealDBConnectionManager::new_with_protocol(
        ConnectionProtocol::Http,  // Specify HTTP protocol
        "127.0.0.1:8000",          // SurrealDB server address (host:port)
        "root",                    // Username for authentication
        "root",                    // Password for authentication
    );
```

| Enum                | Protocol         |
|-------------------------|-------------|
| `ConnectionProtocol::Http` | `http://`   |
| `ConnectionProtocol::Https` | `https://`  |
| `ConnectionProtocol::Ws`    | `ws://`     |
| `ConnectionProtocol::Wss`   | `wss://`    |

In some applications, you may need to interact with SurrealDB using different connection protocols simultaneously for performance, scalability, or specific use cases. Here's an example of how you can easily manage multiple protocols by creating separate connection managers and pools for each:
```rust

    // Import both connection manager and protocol enum
    use mobc_surrealdb::{ConnectionProtocol , SurrealDBConnectionManager};

    // WS Connection Manager
    let ws_manager = SurrealDBConnectionManager::new_with_protocol(
        ConnectionProtocol::Ws, // Specify WS protocol
        "127.0.0.1:8000",       // SurrealDB server address (for WS)
        "root",                 // SurrealDB username
        "root",                 // SurrealDB password
    );

    // HTTP Connection Manager
    let http_manager = SurrealDBConnectionManager::new_with_protocol(
        ConnectionProtocol::Http, // Specify HTTP protocol
        "127.0.0.1:8000",         // SurrealDB server address (for HTTP)
        "root",                   // SurrealDB username
        "root",                   // SurrealDB password
    );

    // WebSocket (WS) Connection Pool
    let ws_pool = Pool::builder()
        .max_open(10)
        .max_idle(2)
        .max_lifetime(Some(Duration::from_secs(300)))
        .build(ws_manager);

    // HTTP Connection Pool
    let http_pool = Pool::builder()
        .max_open(10)
        .max_idle(2)
        .max_lifetime(Some(Duration::from_secs(300)))
        .build(http_manager);

```

## Note
If you're using the [example](#examples) above, make sure to include the necessary dependencies in your project. You can add them easily using the following command:

```bash
cargo add serde
cargo add surrealdb
cargo add mobc
cargo add mobc-surrealdb
```
here is the code that you need to create the required connection pool. Feel free to configure it as needed. This code only creates the connection pool without interacting with the database, i.e., 'storing and retrieving' data.
```rust
// Import necessary crates and modules
use mobc::Pool; // The mobc connection pool
use mobc_surrealdb::SurrealDBConnectionManager;  // The connection manager
use std::time::Duration; // For configuring pool settings
use tokio; // Tokio runtime for asynchronous execution

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the connection manager without specifying a protocol.
    // By default, it will use WebSocket (ws) as the connection protocol.
    let manager = SurrealDBConnectionManager::new(
        "127.0.0.1:8000", // SurrealDB server address
        "root",           // SurrealDB Default Username
        "root",           // SurrealDB Default Password
    );

    // Build the connection pool with custom settings:
    // - Maximum of 20 open connections.
    // - Maximum of 5 idle connections.
    // - Each connection lives for up to 300 seconds.
    // You can keep connections open indefinitely by using ".max_lifetime(None)".
    let pool = Pool::builder()
        .max_open(20)
        .max_idle(5)
        .max_lifetime(Some(Duration::from_secs(300)))
        .build(manager);
    Ok(())
}

```
