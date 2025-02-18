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
