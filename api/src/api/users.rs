use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid as Uid;

use crate::{
  error::ApiError,
  // models::comment::{Comment, NewCommentPayload},
  // schema::{comments, comments::dsl::comments as comments_dsl},
  SharedState,
};
