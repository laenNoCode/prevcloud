use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
pub mod database;
pub mod schema;
mod auth;
mod config;
use auth::create_user::create_user;
use auth::login::login;


#[tokio::main]
async fn main() {
    //get the config
    let conf : config::config::Config = config::config::get_config("config.json".to_string());
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/login", post(login));
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], conf.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

