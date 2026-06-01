#![allow(unused)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
