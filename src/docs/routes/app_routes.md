# app_routes

Configures and returns the main router of the application using the Axum framework.

This function creates a new router (`Router`) that can be used to define and organize the routes of your application. Currently, the router is empty, but it can be expanded with routes as needed.

## Returns

- `Router`: A new Axum router.

## Examples

```rust
use your_project::helpers::app_routes;
use axum::Server;

#[tokio::main]
async fn main() {
    let app = app_routes();

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```