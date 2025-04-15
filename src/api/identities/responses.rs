use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::database::models::Identity;

pub enum ApiResponse {
    OK,
    Created,
    NoContent,
    JsonData(Vec<Identity>),
}
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::NoContent => (StatusCode::NO_CONTENT).into_response(),
            Self::JsonData(json) => (StatusCode::OK, Json(json)).into_response(),
        }
    }
}

pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
