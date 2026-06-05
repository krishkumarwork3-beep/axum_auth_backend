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
    .route("/name", put(update_user_name))
    .route("/role", put(update_user_role))
    .route("/password", put(update_user_password))
}

pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>
) -> Result<impl IntoResponse, HttpError> {

    let filtered_user = FilterUserDto::filter_user(&user.user);
    
    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filtered_user,
        }
    };

    Ok(Json(response_data))
}

pub async fn get_users(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state.db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = app_state.db_client
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

     let response = UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        results: user_count,
    };
}