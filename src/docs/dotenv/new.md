# Dotenv::new

Initializes the dotenv loader.

This function loads environment variables from a `.env` file
into the application's runtime environment. If the file is
missing or an error occurs, the function silently ignores it.

## Examples

```rust
Dotenv::new();
```