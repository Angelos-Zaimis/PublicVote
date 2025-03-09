use crate::api;
use crate::api::routes::routers;
use axum::Router;

pub async fn start_webserver() {
    let app: Router = routers();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.expect("Failed to create lister");
    println!("Web Server running at port 8000...");

    axum::serve(listener,app).await.expect("Failed to start web server");

}