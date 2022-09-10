// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Result};
use from_as::*;

use graph_rs_sdk::oauth::OAuth;

use warp::{http::Response, Filter};

// Client Credentials Grant
// If you have already given admin consent to a user you can skip
// browser authorization step and go strait to requesting an access token.
// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
      // If this is not the first time you are using the client credentials grant
    // then you only have to run request_access_token() and you can comment out
    // what is below.
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,))
        });

    let routes = warp::get().and(warp::path("redirect")).and(query).map(
        |cc: Option<AccessCode>| match cc {
            Some(access_code) => {
                // Print out for debugging purposes.
                println!("CODE: {:#?}", access_code.code);

                // Request an access token.
                set_and_req_access_code(access_code);

                // Generic login page response.
                Response::builder().body(String::from(
                    "Successfully Logged In! You can close your browser.",
                ))
            }
            None =>  {
                Response::builder().body(String::from("There was an issue getting the access code."))
            },
        },
    );

    // Get the oauth client and request a browser sign in
    let mut oauth = get_oauth_client();
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
  Ok(())
}

pub fn show_tasks(json: &bool) -> Result<()> {
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
        .add_scope("Tasks.ReadWrite")
        .redirect_uri("http://localhost:8000/redirect")
        .response_type("code")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf");
    oauth
}

fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = get_oauth_client();
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    println!("HERE");
    oauth.access_code(access_code.code.as_str());
    println!("HERE1");
    
    let mut request = oauth.build().authorization_code_grant();
    println!("HERE2");

    let access_token = request.access_token().send().unwrap();
    println!("HERE3");
    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
    oauth
        .as_file("./auth/web_oauth.json")
        .unwrap();
}