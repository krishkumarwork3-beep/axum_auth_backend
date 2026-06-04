use std::sync::Arc;

use axum::{extract::Query, http::{header, HeaderMap, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}, Extension, Json, Router};
use axum_extra::extract::cookie::Cookie;
use chrono::{Utc, Duration};
use validator::Validate;

use crate::{db::UserExt, dtos::{ForgotPasswordRequestDto, LoginUserDto, RegisterUserDto, ResetPasswordRequestDto, Response, UserLoginResponseDto, VerifyEmailQueryDto}, error::{ErrorMessage, HttpError}, mail::mails::{send_forgot_password_email, send_verification_email, send_welcome_email}, utils::{password, token}, AppState};

pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>
) -> Result<impl IntoResponse, HttpError> {
     body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

}