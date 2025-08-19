use std::{error::Error, intrinsics::atomic_cxchgweak_release_seqcst, net::ToSocketAddrs};
use axum::{http::StatusCode,
     Json,
    Router,
    routing::{get, post}};
use serde::{Serialize, Deserialize};




#[tokio::main]
async fn main() {
tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", post(_))
        .route("", get(|| async )


    
}
