use std::sync::Arc;

use axum::{extract::Query, middleware, response::IntoResponse, routing::{get, put}, Extension, Json, Router};
use validator::Validate;

use crate::{db::UserExt, dtos::{FilterUserDto, NameUpdateDto, RequestQueryDto, Response, RoleUpdateDto, UserData, UserListResponseDto, UserPasswordUpdateDto, UserResponseDto}, error::{ErrorMessage, HttpError}, middleware::{role_check, JWTAuthMiddeware}, models::UserRole, utils::password, AppState};

pub fn users_handler() -> Router {
    
}