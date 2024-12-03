use diesel::prelude::*;
use crate::env::Dotenv;

#[doc = include_str!("docs/db/establish_connection.md")]
/// Establishes a connection to the PostgreSQL database.
///
/// This function retrieves the `DATABASE_URL` environment variable using `Dotenv::get`
/// and attempts to establish a connection to the PostgreSQL database using Diesel's `PgConnection`.
///
/// **Note:** Ensure that the environment variables are loaded (e.g., by calling `Dotenv::new()`) before using this function.
///
/// # Panics
///
/// * If the `DATABASE_URL` environment variable is not found.
/// * If the connection to the database cannot be established.
///
/// # Returns
/// PgConnection
pub fn establish_connection() -> PgConnection {
    let database_url = Dotenv::get("DATABASE_URL").expect("DATABASE_URL not found");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::panic;

    /// Helper function to clear the `DATABASE_URL` environment variable.
    fn clear_database_url() {
        env::remove_var("DATABASE_URL");
    }

    /// Get real DATABASE_URL
    fn get_database_url() -> String {
        Dotenv::new();
        Dotenv::get("DATABASE_URL").expect("DATABASE_URL not found")
    }

    #[test]
    fn test_establish_connection_success() {
        // Setup: Define a valid DATABASE_URL environment variable.
        let database_url = get_database_url();
        env::set_var("DATABASE_URL", database_url);

        // Execute: Attempt to establish a connection.
        let result = panic::catch_unwind(|| {
            establish_connection();
        });

        // Verify: Ensure the connection was established without panicking.
        assert!(result.is_ok());

        // Cleanup
        clear_database_url();
    }
    
    #[test]
    fn test_establish_connection_missing_database_url() {
        // Ensure the DATABASE_URL is not set.
        clear_database_url();

        // Execute: Attempt to establish a connection and expect a panic.
        let result = panic::catch_unwind(|| {
            establish_connection();
        });

        // Verify: Ensure the function panicked.
        assert!(result.is_err());

        // Optionally, check the panic message.
        if let Err(err) = result {
            let panic_msg = if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                String::from("Unknown panic message")
            };
            assert_eq!(panic_msg, "DATABASE_URL not found: \"Environment variable `DATABASE_URL` not found\"");
        }
    }

    #[test]
    fn test_establish_connection_invalid_database_url() {
        // Setup: Define an invalid DATABASE_URL environment variable.
        let invalid_database_url = "invalid_url";
        env::set_var("DATABASE_URL", invalid_database_url);

        // Execute: Attempt to establish a connection and expect a panic.
        let result = panic::catch_unwind(|| {
            establish_connection();
        });

        // Verify: Ensure the function panicked.
        assert!(result.is_err());

        // Cleanup
        clear_database_url();
    }
}
