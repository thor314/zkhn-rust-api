#![allow(unused_imports)]

use std::{process, time};

use db::models::user::User;
use reqwest::{Client, RequestBuilder, Response};
use serial_test::serial;

const WEBSERVER_URL: &str = "http://localhost:8000";

trait ClientExt {
  async fn send_json(self, payload: impl serde::Serialize) -> Response;
  async fn send_empty(self) -> Response;
}

impl ClientExt for RequestBuilder {
  async fn send_json(self, payload: impl serde::Serialize) -> Response {
    self.json(&payload).send().await.unwrap()
  }

  async fn send_empty(self) -> Response { self.send().await.unwrap() }
}

struct ChildGuard {
  child: process::Child,
}

// todo: need to drop the server, this child guard ain't getting it done
// hacky solution:
// $ ps # look for "server"
// $ kill <pid> # or pkill server
//
// failing to drop the shuttle server:
// drop(_child_guard);
impl Drop for ChildGuard {
  fn drop(&mut self) {
    self.child.kill().expect("Failed to kill child process");
    self.child.wait().expect("Failed to wait for child process to exit");
    // todo(lies!)
    println!("💀 Killed child process 💀");
  }
}

/// Run the shuttle server
async fn cargo_shuttle_run() -> ChildGuard {
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

// commented for speed and convenience
// #[tokio::test]
// #[serial]
// async fn create_get() {
//   // run the server
//   let mut _child_guard = cargo_shuttle_run().await;

//   // create a client that stores cookies
//   let client = Client::builder().cookie_store(true).build().unwrap();

//   // create the default user
//   let payload = api::UserPayload::default();
//   let res = client.post(format!("{}/users", WEBSERVER_URL)).send_json(&payload).await;
//   assert_eq!(res.status(), 200);

//   // get the user
//   let res = client.get(format!("{}/users/alice", WEBSERVER_URL)).send_empty().await;
//   assert_eq!(res.status(), 200);
//   let body: User = res.json().await.unwrap();
//   assert!(body.username.0 == "alice");
// }

#[tokio::test]
#[serial]
async fn login_test() {
  // run the server
  let mut _child_guard = cargo_shuttle_run().await;

  // create a client that stores cookies
  let client = Client::builder().cookie_store(true).build().unwrap();

  // Log in with invalid credentials.
  let payload = api::CredentialsPayload::new("ferris", "hunter42", None);
  let res = client.post(format!("{}/users/login", WEBSERVER_URL)).send_json(payload).await;
  assert_eq!(res.status(), 401);
  assert_eq!(res.url().to_string(), format!("{}/users/login", WEBSERVER_URL));

  // create the default user
  let payload = api::UserPayload::default();
  let _res = client.post(format!("{}/users", WEBSERVER_URL)).send_json(&payload).await;
  // assert_eq!(_res.status(), 200);

  // Log in with valid credentials.
  let payload = api::CredentialsPayload::default();
  let res = client.post(format!("{}/users/login", WEBSERVER_URL)).send_json(&payload).await;
  // dbg!(&res);
  assert_eq!(res.status(), 200);
  assert_eq!(res.url().to_string(), format!("{}/users/login", WEBSERVER_URL));

  // Log out and check the cookie has been removed in response.
  let res = client.post(format!("{}/users/logout", WEBSERVER_URL)).send_empty().await;
  assert_eq!(res.status(), 200);
  assert!(res.cookies().find(|c| c.name() == "id").is_some_and(|c| c.value() == ""));

  let res = client.post(format!("{}/users/logout", WEBSERVER_URL)).send_empty().await;
  assert_eq!(res.status(), 200);
}

// for cookie in res.cookies() {
//   println!("{:?}", cookie);
// }
