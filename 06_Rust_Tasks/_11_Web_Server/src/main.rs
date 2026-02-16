pub mod model;
pub mod api;
pub mod handler;
pub mod routes;

use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use handler::load_drivers;
use routes::create_routes;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let drivers = load_drivers();

    let state = Arc::new(RwLock::new(drivers));

    let app = create_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4500));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
