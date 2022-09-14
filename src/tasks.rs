// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Result};
use from_as::*;
use std::fs::File;
use std::io::{Read, Write};
use graph_rs_sdk::oauth::OAuth;
use graph_rs_sdk::prelude::*;

use warp::Filter;

// Client Credentials Grant
// If you have already given admin consent to a user you can skip
// browser authorization step and go strait to requesting an access token.
// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessCode {
    //admin_consent: bool,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub state: String,
    pub id: u32,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, text: String) -> Task {
        let updated_at: DateTime<Utc> = Utc::now();
        let state = "todo".to_string();
        Task { text, state, id, updated_at }
    }
}

#[tokio::main]
pub async fn login() -> Result<()> {
  println!("Loggin In");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
      // If this is not the first time you are using the client credentials grant
    // then you only have to run request_access_token() and you can comment out
    // what is below.
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,))
        });

    let routes = warp::get().and(warp::path("redirect")).and(query).map(
        move |cc: Option<AccessCode>| match cc {
            Some(access_code) => {
                // Print out for debugging purposes.
                // println!("CODE: {:#?}", access_code.code);
                tx.send(access_code.code).unwrap();
                Ok(warp::reply::with_status("Hello from <b>tdi</b> - the access code was received and stored locally, you may safely close this browser window!", http::status::StatusCode::CREATED))
            }
            None =>  {
                tx.send("error getting access code".to_string()).unwrap();
                //Response::builder().body(String::from("There was an issue getting the access code."))
                Ok(warp::reply::with_status("Hello from <b>tdi</b> - error encountered requesting the access code.", http::status::StatusCode::NOT_FOUND))
            },
        },
    );

    // Get the oauth client and request a browser sign in
    let mut oauth = get_oauth_client();
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    let server = warp::serve(routes)
      .bind_with_graceful_shutdown(
                  ([127, 0, 0, 1], 8000),
                  async move { 
                    let code = rx.recv().await.unwrap(); 
                    req_access_token(code).await;
                  })
      .1;

    server.await;

  Ok(())
}

pub fn show_me(json: &bool) -> Result<()> {
  let token = read_access_token();
  let client = Graph::new(&token);
  match client.v1()
    .me().get_user().send() {
      Ok(res) => {
        if *json {
          println!("Me as JSON: {:?}", serde_json::to_string(res.body()).unwrap());
        } else {
          println!("Me as a table: {:?}", res.body());
        }
      },
      Err(err) => println!("Error: {}", err),
    }
  Ok(())
}

pub fn show_tasks(json: &bool) -> Result<()> {
  // let token = read_access_token();
  // let client = Graph::new(&token);
  // let response = client.v1()
  //   .me().get_user().send();
  // println!("response: {:?}", response);
  let tasks = collect_tasks();
  if *json {
    println!("Tasks as JSON: {}", serde_json::to_string(&tasks.unwrap()).unwrap());
  } else {
    println!("Tasks as a table: {:?}", tasks);
  }
  Ok(())
}

pub fn add_task(new_task: &String) -> Result<()> {
  let task = Task::new(99, new_task.to_string());
  println!("Adding new task: {:?}", task);
  Ok(())
}

pub fn complete_task(id: &u32) -> Result<()> {
  println!("Completing task: {}", id);
  Ok(())
} 

pub fn reopen_task(id: &u32) -> Result<()> {
  println!("Reopening task: {}", id);
  Ok(())
} 

pub fn delete_task(id: &u32) -> Result<()> {
  println!("Deleting task: {}", id);
  Ok(())
} 

fn collect_tasks() -> Result<Vec<Task>> {
    let tasks = vec![];
    Ok(tasks)
}

fn get_oauth_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("tasks.readwrite")
        .add_scope("tasks.read")
        .add_scope("user.read")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
    oauth
}

async fn req_access_token(code: String) {
  //let mut oauth = get_oauth_client();
  let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
    //    .client_secret(CLIENT_SECRET)
        .add_scope("tasks.readwrite")
        .add_scope("tasks.read")
        .add_scope("user.read")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
    
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
  oauth.access_code(code.as_str());
    
  let mut request = oauth.build_async().authorization_code_grant();
    let access_token = 
      match request.access_token().send().await {
        Ok(res) => res,
        Err(err) => {
          println!("tdi login error: {:?}", err);
          std::process::exit(1);
        }
      };

  oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
  // println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
  oauth
    .as_file("./tdi.json")
    .unwrap();

  println!("tdi: logged in, and stored token for future use.");
}

fn read_access_token() -> String {
  let mut file = File::open("./.tokentdi").expect("Error opening File");
  let mut code = String::new();
  file.read_to_string(&mut code).expect("error reading code from .code");
  // println!("T: {}", code);
  code
}

