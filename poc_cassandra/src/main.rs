use scylla::{Session, SessionBuilder};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    let que = format!("Select * from my_keyspace.users");
    let res = session.query(que, &[]).await?;

    println!("{:?}",res);


    Ok(())
}


/*
use scylla::{Session, SessionBuilder};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the Cassandra cluster
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    // Create the keyspace
    let create_keyspace = "
        CREATE KEYSPACE IF NOT EXISTS my_keyspace
        WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };
    ";
    session.query(create_keyspace, &[]).await?;

    // Use the keyspace
    let use_keyspace = "USE my_keyspace;";
    session.query(use_keyspace, &[]).await?;

    // Create the users table
    let create_table = "
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            name TEXT,
            email TEXT,
            password TEXT
        );
    ";
    session.query(create_table, &[]).await?;

    // Insert some dummy data
    let insert_data = "
        INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Alice', 'alice@example.com', 'password1');
        INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Bob', 'bob@example.com', 'password2');
        INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Charlie', 'charlie@example.com', 'password3');
    ";
    session.query(insert_data, &[]).await?;

    println!("Keyspace and table created, and data inserted successfully.");

    Ok(())
}
*/