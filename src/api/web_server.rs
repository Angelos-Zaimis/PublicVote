use axum::{routing::post, Json, Router};

pub async fn start_webserver() {
    let app: Router = Router::new()
        .route("/vote", post(create_vote));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.expect("Failed to create lister");
    println!("Server running at http://{:?}", listener);

    axum::serve(listener,app).await.expect("Failed to start web server");

}

async fn create_vote(
    Json(payload): Json<VoteRequest>,
) -> Json<String> {
    let block = blockchain.add_block(payload.vote);
    Json("You voted successfully!")
}