use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

use crate::errors::HttpAppError::LockError;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;

pub type AppResponse = Result<HttpResponse, HttpAppError>;

#[derive(Debug, Error)]
pub enum HttpAppError {
    #[error("An element with the same ID already exists")]
    Conflict,
    #[error("Not found")]
    NotFound,
    #[error("Poison error {0}")]
    LockError(String),
}

impl ResponseError for HttpAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpAppError::Conflict => StatusCode::CONFLICT,
            HttpAppError::NotFound => StatusCode::NOT_FOUND,
            HttpAppError::LockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for HttpAppError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        LockError("Read Lock was poisoned".to_string())
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for HttpAppError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        LockError("Write Lock was poisoned".to_string())
    }
}
