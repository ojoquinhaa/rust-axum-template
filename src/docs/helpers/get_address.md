# get_address

Retrieves the server address by combining the `HOST` and `PORT` environment variables.

This function fetches the `HOST` and `PORT` environment variables using the `Dotenv` utility.
If `HOST` is not set, it defaults to `"127.0.0.1"`. If `PORT` is not set, it defaults to `"3000"`.
The function then formats and returns the address in the form `"HOST:PORT"`.

## Returns

A `String` containing the server address in the format `"HOST:PORT"`.

## Examples

```rust
let address = helpers::get_address();
println!("Server is running at {}", address);
```