use axum::{extract::State, http::StatusCode, Json};

// use sqlx::types::Uuid;
use crate::{
  error::ApiError,
  // models::comment::{Comment, NewCommentPayload},
  // schema::{comments, comments::dsl::comments as comments_dsl},
  SharedState,
};
