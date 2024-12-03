# establish_connection

Establishes a connection to the PostgreSQL database.

This function retrieves the DATABASE_URL environment variable using Dotenv::get and attempts to establish a connection to the PostgreSQL database using Diesel's PgConnection.

## Returns

A PgConnection object representing the established database connection.

## Panics

If the DATABASE_URL environment variable is not found.
If the connection to the database cannot be established.

## Examples

```rust
use crate::db::establish_connection;
use crate::env::Dotenv;

fn main() {
    // Load environment variables from the .env file
    Dotenv::new();

    // Establish a database connection
    let connection = establish_connection();

    println!("Successfully connected to the database.");
}
```