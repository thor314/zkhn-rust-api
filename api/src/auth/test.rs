use std::error::Error;

use axum::{body::Body, Router};
use axum_login::{login_required, AuthManagerLayerBuilder};
use sqlx::{FromRow, PgPool};
use tower::ServiceExt;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::{app, error::ApiError};

async fn setup(pool: PgPool) -> Result<Router, ApiError> {
  let app = app(pool).await?;
  Ok(app)
}

// // we can fetch the default user, ferris
// #[sqlx::test]
// async fn test_db(pool: PgPool) {
//     let mut conn = pool.acquire().await.unwrap();

//     let users = sqlx::query("SELECT * FROM users where username = $1")
//         .bind("ferris")
//         .fetch_one(&mut *conn)
//         .await
//         .unwrap();
//     let ferris: User = User::from_row(&users).unwrap();
//     assert!(ferris.username == "ferris");
// }

// #[sqlx::test]
// async fn test_login_logout(pool: PgPool) {
//     let app = setup_app(pool).await.unwrap();

//     let make_login_request = |username: &str, password: &str| {
//         let credentials = Credentials {
//             username: username.to_string(),
//             password: password.to_string(),
//             next: None,
//         };
//         let credentials = serde_urlencoded::to_string(credentials).unwrap();

//         Request::builder()
//             .uri("/login")
//             .method("POST")
//             .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
//             .body(credentials)
//             .unwrap()
//     };

//     let valid_login_request = make_login_request("ferris", "hunter42");
//     let login_response = app.clone().oneshot(valid_login_request).await.unwrap();
//     dbg!(&login_response);
//     assert_eq!(login_response.status(), StatusCode::SEE_OTHER);

//     // login with invalid credentials
//     let invalid_login_request = make_login_request("ferris", "huntouer24");
//     let login_response = app.clone().oneshot(invalid_login_request).await.unwrap();
//     dbg!(&login_response);
//     assert_eq!(login_response.status(), StatusCode::SEE_OTHER);

//     let logout_request = Request::builder()
//         .uri("/logout")
//         .method("GET")
//         .body(Body::empty())
//         .unwrap();
//     let logout_response = app.clone().oneshot(logout_request).await.unwrap();
//     dbg!(&logout_response);
//     assert_eq!(logout_response.status(), StatusCode::SEE_OTHER);
// }
