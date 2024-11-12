mod config;
mod db;
mod dtos;
mod error;
mod models;
mod utils;

use crate::db::DBClient;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{Extension, Router};
use config::Config;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        //.allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db_client = DBClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    let app = Router::new().layer(Extension(app_state)).layer(cors);

    println!(
        "{}",
        format!("🚀 Server is running on 0.0.0.0:{}", config.port)
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
