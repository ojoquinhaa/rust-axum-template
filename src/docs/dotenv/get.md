# Dotenv::get

Retrieves the value of an environment variable.

This function fetches the value of a given environment variable by its key.
If the variable is not found, it returns an error with a descriptive message.

## Arguments

- `key`: A string slice that holds the name of the environment variable.

## Returns

A `Result` which is:
- `Ok(String)` containing the value of the environment variable if found.
- `Err(String)` with an error message if the variable is not set.

## Examples

```rust
let value = Dotenv::get("DATABASE_URL").expect("DATABASE_URL not found");
println!("Database URL: {}", value);
```