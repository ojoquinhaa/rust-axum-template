mod routes;
mod env;
mod helpers;
mod db;
mod schema;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Inicia o Dotenv
    env::Dotenv::new();

    // Configura as rotas do aplicativo
    let app: axum::Router = routes::app_routes();

    // Monta o endereço e cria o listener
    let addr: String = helpers::get_address();
    let listener: TcpListener = TcpListener::bind(&addr).await.unwrap();

    // Callback
    tracing::info!("Server listening at: {}", addr);

    axum::serve(listener, app).await.unwrap();
}
