use axum::Router;

/// Configures and returns the main router of the application using the Axum framework.
#[doc = include_str!("../docs/routes/app_routes.md")]
pub fn app_routes() -> Router {
    Router::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for the `oneshot` method

    /// Tests if `app_routes` returns a valid router.
    #[tokio::test]
    async fn test_app_routes_creation() {
        let router = app_routes();

        // Since the router is empty, any request should return 404 Not Found
        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    /// Tests if the router can handle specific routes (when added).
    #[tokio::test]
    async fn test_app_routes_with_routes() {
        use axum::routing::get;

        // Define a test handler as async
        async fn handler() -> &'static str {
            "Hello, World!"
        }

        // Create a router with a GET route for "/hello"
        let router = Router::new().route("/hello", get(handler));

        // Test the "/hello" route
        let response = router
            .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Verify the response body
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, "Hello, World!");
    }
}
