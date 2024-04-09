use std::{process, process::Command, time};

use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;

pub const WEBSERVER_URL: &str = "http://localhost:8000";

/// convenience function to send a request and check the response status
pub async fn send(
  client: &Client,
  payload: impl serde::Serialize,
  method: &str,
  path: &str,
  status: u16,
  tag: &str,
) -> Response {
  let res = match method {
    "POST" => client.post(format!("{}/{}", WEBSERVER_URL, path)).send_json(payload).await,
    "PUT" => client.put(format!("{}/{}", WEBSERVER_URL, path)).send_json(payload).await,
    "PUT_EMPTY" => client.put(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    "GET" => client.get(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    "DELETE" => client.delete(format!("{}/{}", WEBSERVER_URL, path)).send_empty().await,
    _ => panic!("Invalid method"),
  };
  assert_eq!(res.status(), status, "Test {} failed", tag);
  res
}

pub async fn send_get<T: DeserializeOwned>(
  client: &Client,
  payload: impl serde::Serialize,
  method: &str,
  path: &str,
  status: u16,
  tag: &str,
) -> T {
  send(client, payload, method, path, status, tag).await.json().await.unwrap()
}

/// Run the shuttle server
pub async fn cargo_shuttle_run() -> ChildGuard {
  // tracing_subscriber_setup();
  // db_setup();
  server_cleanup();
  rm_docker_claude();
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
fn server_cleanup() {
  Command::new("pkill").arg("server").spawn().expect("Failed to kill server");
  println!("Killed test server");
}

/// migrate the db to the newest schema
fn _db_setup() {
  Command::new("sqlx")
    .arg("db")
    .arg("reset")
    .arg("-y")
    .current_dir("../db")
    .spawn()
    .expect("failed to reset database");
  println!("test database setup");
}

/// Remove existing Docker container
fn rm_docker_claude() {
  let output = Command::new("docker")
    .args(["ps", "--quiet", "--filter", "name=shuttle_tk-shuttle-zkhn-rust-api3_shared_postgres"])
    .output()
    .expect("Failed to execute docker ps command");

  let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

  if !container_id.is_empty() {
    let _ = Command::new("docker")
      .args(["rm", "-f", &container_id])
      .output()
      .expect("Failed to remove Docker container");
  }
}

fn _rm_docker_gemini() {
  let output = Command::new("docker")
    .args(["ps", "-q", "-f", "name=shuttle_tk-shuttle-zkhn-rust-api3_shared_postgres"])
    .output()
    .unwrap();

  if !output.stdout.is_empty() {
    let container_id = String::from_utf8(output.stdout).unwrap().trim().to_string();
    Command::new("docker").args(["rm", "-f", &container_id]).output().unwrap();
  }
}

// fn tracing_subscriber_setup() {
//   // let filter = tracing_subscriber::EnvFilter::from_default_env()
//   // tracing_subscriber::fmt().with_env_filter(filter).with_test_writer().init();
//   let sub = tracing_subscriber::fmt()
//     .with_env_filter(
//       tracing_subscriber::EnvFilter::from_default_env()
//     .add_directive("sqlx=warn".parse().unwrap())
//     // .add_directive("tower_http=debug".parse().unwrap())
//     // .add_directive("tower_sessions=debug".parse().unwrap())
//     // .add_directive("axum_login=debug".parse().unwrap())
//     .add_directive("axum_login=info".parse().unwrap())
//     .add_directive("h2=info".parse().unwrap())
//     .add_directive("api=debug".parse().unwrap())
//     .add_directive("db=debug".parse().unwrap())
//     .add_directive("server=debug".parse().unwrap()),
//     )
//     .with_test_writer()
//     .finish();
//   // .init();
//   tracing::subscriber::set_global_default(sub).unwrap();
// }
