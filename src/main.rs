#![allow(unused)]//use it in first line for safe implementation 
mod models;
mod config;
mod dtos;
mod error;
mod db;

use std::sync::Arc;

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

fn main() {
    println!("Hello, world!");
}
