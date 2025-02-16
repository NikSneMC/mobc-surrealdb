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
        "root".to_string(),           // Username
        "root".to_string(),           // Password
    );

    // Build the connection pool with specified settings
    let pool = Pool::builder()
        .max_open(1) // Maximum number of open connections
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

    // Retrieve all records from the "person" table
    let mut response = conn.query("SELECT * FROM person").await?;
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
