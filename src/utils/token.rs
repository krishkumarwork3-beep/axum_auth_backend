use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorMessage, HttpError};