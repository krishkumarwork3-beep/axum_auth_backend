use std::sync::Arc;

use axum::{extract::Query, middleware, response::IntoResponse, routing::{get, put}, Extension, Json, Router};
use validator::Validate;

use crate::{db::UserExt, dtos::{FilterUserDto, NameUpdateDto, RequestQueryDto, Response, RoleUpdateDto, UserData, UserListResponseDto, UserPasswordUpdateDto, UserResponseDto}, error::{ErrorMessage, HttpError}, middleware::{role_check, JWTAuthMiddeware}, models::UserRole, utils::password, AppState};

pub fn users_handler() -> Router {
    Router::new()
    .route(
            "/me", 
            get(get_me)
            .layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin, UserRole::User])
            }))
    )
    .route(
        "/users", 
        get(get_users)
        .layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![UserRole::Admin])
        }))
    )
}