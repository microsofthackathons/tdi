// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use directories::ProjectDirs;
use from_as::*;
use graph_rs_sdk::oauth::OAuth;
use serde::{Deserialize, Serialize};
use std::io::Result;
use warp::Filter;

// Client Credentials Grant
// If you have already given admin consent to a user you can skip browser authorization step and go strait to requesting an access token. The client_id
// and client_secret must be changed before running this example.
static CLIENT_ID: &str = "f7adafb9-4354-4008-8707-63776b430fc9";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessCode {
    //admin_consent: bool,
    code: String,
}

pub fn get_oauth_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .add_scope("tasks.readwrite")
        .add_scope("tasks.read")
        .add_scope("user.read")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
    oauth
}

pub async fn req_access_token(code: String) {
    //let mut oauth = get_oauth_client();
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .add_scope("tasks.readwrite")
        .add_scope("tasks.read")
        .add_scope("user.read")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");

    // The response type is automatically set to token and the grant type is
    // automatically set to authorization_code if either of these were not
    // previously set. This is done here as an example.
    oauth.access_code(code.as_str());

    let mut request = oauth.build_async().authorization_code_grant();
    let access_token = match request.access_token().send().await {
        Ok(res) => res,
        Err(err) => {
            println!("tdi login error: {:?}", err);
            std::process::exit(1);
        },
    };

    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access
    // Token. println!("{:#?}", &oauth);

    match std::fs::create_dir_all(get_config_dir()) {
        Ok(()) => {
            println!("tdi: creating directory path for access token config.")
        },
        Err(_) => {
            println!("tdi: error created directory path for access token config.");
            std::process::exit(1);
        },
    }
    let config_path = get_config_dir() + "/tdi.json";
    // Save our configuration to a file so we can retrieve it from other requests.
    oauth.as_file(config_path).unwrap();

    println!(
        "tdi: logged in, and stored token for future use at {}.",
        get_config_dir()
    );
}

pub fn read_access_token() -> String {
    let data = std::fs::read_to_string(get_config_dir() + "/tdi.json")
        .expect("tdi: unable to read access token configuration.");
    let res: serde_json::Value =
        serde_json::from_str(&data).expect("tdi: unnable to parse configuration.");
    res["access_token"]["access_token"].to_string()
}

fn get_config_dir() -> String {
    let proj_dirs = ProjectDirs::from("com", "microsofthackathons", "tdi");
    let config_dir = proj_dirs.unwrap().config_dir().to_path_buf();
    config_dir.into_os_string().into_string().unwrap()
}

#[tokio::main]
pub async fn login() -> Result<()> {
    println!("tdi: authenticating, a browser window will open.");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    // If this is not the first time you are using the client credentials grant
    // then you only have to run request_access_token() and you can comment out
    // what is below.
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

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
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 8000), async move {
            let code = rx.recv().await.unwrap();
            req_access_token(code).await;
            std::process::exit(0);
        })
        .1;

    server.await;

    Ok(())
}

pub fn logout() -> Result<()> {
    println!("tdi: logging out of Microsoft SSO");
    let mut oauth: OAuth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
        .post_logout_redirect_uri("http://localhost:8000/redirect");
    oauth.v1_logout().unwrap();

    std::fs::remove_file(get_config_dir() + "/tdi.json")?;
    // TODO: remove the OAuth authorization and delete the locally stored cred
    Ok(())
}
