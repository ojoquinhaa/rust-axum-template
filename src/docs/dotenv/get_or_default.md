# Dotenv::get_or_default

Retrieves the value of an environment variable or returns a default value.

This function attempts to fetch the value of a given environment variable
by its key. If the variable is not set, it falls back to the provided default value.

## Arguments

- `key`: A string slice that holds the name of the environment variable.
- `default`: A string slice that holds the default value to return if the variable is not set.

## Returns

A `String` containing:
- The value of the environment variable if found.
- The provided default value if the variable is not set.

## Examples

```rust
let host = Dotenv::get_or_default("HOST", "127.0.0.1");
println!("Host: {}", host);
```