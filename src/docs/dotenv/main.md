# Dotenv

A utility struct for managing environment variables using dotenv.

This struct provides a simple interface to load environment variables
from a `.env` file and access them within your application.

## Examples

```rust
use dotenv_handler::Dotenv;

// Initialize dotenv (loads .env variables)
Dotenv::new();

// Access an environment variable
let api_key = Dotenv::get("API_KEY").expect("API_KEY not found");

// Access an environment variable with a default fallback
let host = Dotenv::get_or_default("HOST", "localhost");
println!("Host: {}", host);
```