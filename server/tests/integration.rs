#![allow(unused_imports)]

use std::{process, time};

use api::*;
use db::models::user::User;
use reqwest::{Client, RequestBuilder, Response};
use serial_test::serial;
use tracing::info;

use self::integration_utils::{cargo_shuttle_run, db_setup, ChildGuard, ClientExt};

const WEBSERVER_URL: &str = "http://localhost:8000";

mod integration_utils {
  use super::*;
  pub trait ClientExt {
    async fn send_json(self, payload: impl serde::Serialize) -> Response;
    async fn send_empty(self) -> Response;
  }

  impl ClientExt for RequestBuilder {
    async fn send_json(self, payload: impl serde::Serialize) -> Response {
      self.json(&payload).send().await.unwrap()
    }

    async fn send_empty(self) -> Response { self.send().await.unwrap() }
  }

  pub struct ChildGuard {
    pub child: process::Child,
  }

  // hack - this drops the process, but not the docker container
  // hack - it only maybe drops the process, you may get a broken pipe error
  impl Drop for ChildGuard {
    fn drop(&mut self) {
      self.child.kill().expect("Failed to kill child process");
      self.child.wait().expect("Failed to wait for child process to exit");
      println!("💀 Killed child process 💀");
    }
  }

  /// remove any artifacts of previous tests
  pub fn server_cleanup() {
    process::Command::new("pkill").arg("server").spawn().expect("Failed to kill server");
    println!("Killed test server");
  }

  /// migrate the db to the newest schema
  pub fn db_setup() {
    process::Command::new("sqlx")
      .arg("db")
      .arg("reset")
      .arg("-y")
      .current_dir("../db")
      .spawn()
      .expect("failed to reset database");
    println!("test database setup");
  }

  /// Run the shuttle server
  pub async fn cargo_shuttle_run() -> ChildGuard {
    // todo: how to rm the docker container
    db_setup();
    server_cleanup();
    let child = process::Command::new("cargo")
      .arg("shuttle")
      .arg("run")
      .spawn()
      .expect("Failed to start example binary");

    let start_time = time::Instant::now();
    let mut is_server_ready = false;

    while start_time.elapsed() < time::Duration::from_secs(300) {
      if reqwest::get(WEBSERVER_URL).await.is_ok() {
        is_server_ready = true;
        println!("Server ready, elapsed time: {:?}", start_time.elapsed());
        break;
      }
      tokio::time::sleep(time::Duration::from_secs(1)).await;
    }

    if !is_server_ready {
      panic!("The web server did not become ready within the expected time.");
    }

    ChildGuard { child }
  }
}

/// convenience function to send a request and check the response status
pub async fn send(
  client: &Client,
  payload: impl serde::Serialize,
  method: &str,
  path: &str,
  status: u16,
  tag: &str,
) {
  let res = match method {
    "POST" => client.post(format!("{}/{}", WEBSERVER_URL, path)).send_json(payload).await,
    "PUT" => client.put(format!("{}/{}", WEBSERVER_URL, path)).send_json(payload).await,
    "PUT_EMPTY" => client.put(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    "GET" => client.get(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    "DELETE" => client.delete(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    _ => panic!("Invalid method"),
  };
  assert_eq!(res.status(), status, "Test {} failed", tag);
}

#[tokio::test]
#[serial]
async fn user_crud() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();

  send(&c, "", "GET", "users/alice", 404, "00").await;
  send(&c, UserPayload::default(), "POST", "users", 200, "1").await;
  send(&c, UserPayload::default(), "POST", "users", 409, "2").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "3").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "4").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "5").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "6").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "7").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "8").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "9").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "0").await;
  send(&c, "", "PUT_EMPTY", "users/reset-password-link/alice", 200, "a").await;
  send(&c, "", "PUT_EMPTY", "users/reset-password-link/alice", 200, "b").await;
  send(&c, ChangePasswordPayload::default(), "PUT", "users/change-password", 200, "c").await;
  send(&c, ChangePasswordPayload::default(), "PUT", "users/change-password", 200, "d").await;
  send(&c, "", "GET", "users/alice", 200, "e").await;
}
