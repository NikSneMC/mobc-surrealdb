# mobc-surrealdb

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![crates.io](https://img.shields.io/badge/crates.io-latest-%23dea584)](https://crates.io/crates/mobc-surrealdb)

mobc-surrealdb is an asynchronous connection pool manager for SurrealDB, built on top of the mobc library. It streamlines the management and reuse of database connections, enhancing the efficiency of applications that frequently interact with SurrealDB.

## Features

- **Asynchronous Connection Pooling**: Leverages mobc to manage a pool of connections, reducing the overhead of establishing new connections.
- **Automatic Connection Recycling**: Ensures that connections are validated and reused, maintaining optimal performance.
- **Customizable Pool Settings**: Configure parameters such as maximum open connections, idle connections, and connection lifetime to suit your application's needs.

## Installation

Add `mobc-surrealdb` to your project's `Cargo.toml`:

```toml
[dependencies]
mobc-surrealdb = "0.1.0"
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
use mobc::Pool;
use mobc_surrealdb::SurrealDBConnectionManager;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surrealdb::sql::Thing;
use tokio;

// Define a struct to represent a person, with fields for ID, name, and age
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: Option<Thing>, // The ID is optional and of type Thing
    name: String,      // The name is a string
    age: i64,          // The age is a 64-bit integer
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the connection manager with the SurrealDB server address and credentials
    let manager = SurrealDBConnectionManager::new(
        "127.0.0.1:8000".to_string(), // SurrealDB server address
        "root".to_string(),           // default Username
        "root".to_string(),           // default Password
    );

    // Build the connection pool with specified settings
    let pool = Pool::builder()
        .max_open(20) // Maximum number of open connections
        .max_idle(5) // Maximum number of idle connections
        .max_lifetime(Some(Duration::from_secs(300))) // Maximum lifetime of each connection
        .build(manager);

    // Get a connection from the pool
    let conn = pool.get().await?;

    // Set the context by specifying the namespace and database
    conn.use_ns("accounts").use_db("users").await?;

    // Create a new person instance
    let person = Person {
        id: None,                 // ID is None; it will be assigned by the database
        name: "Alice".to_string(), // Name of the person
        age: 30,                  // Age of the person
    };

    // Insert the person into the "user" table
    let _created: Option<Person> = conn.create("user").content(person).await?;

    // Retrieve all records from the "user" table
    let mut response = conn.query("SELECT * FROM user").await?;
    let persons: Vec<Person> = response.take(0)?;

    // Iterate over each person and print their details
    for person in persons {
        println!("******************");
        // Check if the person has an ID
        if let Some(ref thing) = person.id {
            // Check if the ID is a string
            if let surrealdb::sql::Id::String(ref id_str) = thing.id {
                println!("id: {}", id_str); // Print the ID
            } else {
                println!("id: Non-string ID"); // Handle non-string IDs
            }
        } else {
            println!("id: None"); // Handle the case where ID is None
        }
        println!("name: {}", person.name); // Print the name
        println!("age: {}", person.age);   // Print the age
        println!("******************");
    }
    
    Ok(())
}

```
## Note
If you're using this example, you may need to add the `serde` , `surrealdb` , `mobc` and `mobc-surrealdb` You can add them using the following command:

```bash
cargo add serde
cargo add surrealdb
cargo add mobc
cargo add mobc-surrealdb
```
In case you don't want to use the example above, here is the code you need to create the required connection pool. Feel free to configure it as needed. This code only creates the connection pool without interacting with the database, i.e., 'storing and retrieving' data.
```rust
// Import necessary crates and modules
use mobc::Pool;
use mobc_surrealdb::SurrealDBConnectionManager;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the connection manager with the SurrealDB server address and credentials
    let manager = SurrealDBConnectionManager::new(
        "127.0.0.1:8000".to_string(), // SurrealDB server address
        "root".to_string(),           // default Username
        "root".to_string(),           // default Password
    );

    // Build the connection pool with specified settings
    let pool = Pool::builder()
        .max_open(20) // Maximum number of open connections
        .max_idle(5) // Maximum number of idle connections
        .max_lifetime(Some(Duration::from_secs(300))) // Maximum lifetime of each connection
        .build(manager);

    // Get a connection from the pool
    let _conn = pool.get().await?;

    Ok(())
}

```
