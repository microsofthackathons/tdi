// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use chrono::{serde::ts_seconds, DateTime, Utc};
use clap::Parser;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use graph_rs_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::auth::*;
use crate::cli::Cli;
use crate::cli::Commands::*;

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
        Task {
            text,
            state,
            id,
            updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub display_name: String,
    pub given_name: String,
    pub surname: String,
    pub id: String,
    pub job_title: String,
    pub mail: String,
    pub mobile_phone: String,
    pub office_location: String,
    pub user_principal_name: String,
}

impl User {
    pub fn new(
        display_name: String,
        given_name: String,
        surname: String,
        id: String,
        job_title: String,
        mail: String,
        mobile_phone: String,
        office_location: String,
        user_principal_name: String,
    ) -> User {
        User {
            display_name,
            given_name,
            surname,
            id,
            job_title,
            mail,
            mobile_phone,
            office_location,
            user_principal_name,
        }
    }

    pub fn empty() -> User {
        User {
            display_name: "".to_string(),
            given_name: "".to_string(),
            surname: "".to_string(),
            id: "".to_string(),
            job_title: "".to_string(),
            mail: "".to_string(),
            mobile_phone: "".to_string(),
            office_location: "".to_string(),
            user_principal_name: "".to_string(),
        }
    }
}

pub fn show_me(json: &bool) -> Result<()> {
    let token = read_access_token();
    let client = Graph::new(&token);

    match client.v1().me().get_user().send() {
        Ok(res) => {
            if *json {
                println!(
                    "Me as JSON: {:?}",
                    serde_json::to_string(res.body()).unwrap()
                );
            } else {
                let user = match res.body().as_object() {
                    None => User::empty(),
                    Some(map) => User::new(
                        map.get(&"displayName".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"givenName".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"surname".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"id".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"jobTitle".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"mail".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"mobilePhone".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"officeLocation".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                        map.get(&"userPrincipalName".to_string())
                            .unwrap()
                            .to_string()
                            .replace("\"", ""),
                    ),
                };

                let table = vec![
                    vec![
                        "Display Name".cell().bold(true),
                        user.display_name.cell().justify(Justify::Right),
                    ],
                    vec![
                        "Given Name".cell().bold(true),
                        user.given_name.cell().justify(Justify::Right),
                    ],
                    vec![
                        "Surname".cell().bold(true),
                        user.surname.cell().justify(Justify::Right),
                    ],
                    vec![
                        "ID".cell().bold(true),
                        user.id.cell().justify(Justify::Right),
                    ],
                    vec![
                        "Mail".cell().bold(true),
                        user.mail.cell().justify(Justify::Right),
                    ],
                    vec![
                        "Mobile Phone".cell().bold(true),
                        user.mobile_phone.cell().justify(Justify::Right),
                    ],
                    vec![
                        "Office Location".cell().bold(true),
                        user.office_location.cell().justify(Justify::Right),
                    ],
                    vec![
                        "User Principal Name".cell().bold(true),
                        user.user_principal_name.cell().justify(Justify::Right),
                    ],
                ]
                .table()
                .title(vec!["Items".cell().bold(true), "Details".cell().bold(true)])
                .bold(true);

                match print_stdout(table) {
                    Err(e) => println!("{:?}", e),
                    _ => (),
                }
            }
        }
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
        println!(
            "Tasks as JSON: {}",
            serde_json::to_string(&tasks.unwrap()).unwrap()
        );
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

pub fn interactive() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new().expect("unable to create interactive shell");
    let command = rl.readline("tdi>>");

    loop {
        match command {
            Ok(ref command) => {
                let args: Vec<&str> = command.split_whitespace().collect();
                let command = Cli::try_parse_from(args).expect("unable to parse");
                match &command.command {
                    Some(Login {}) => login(),
                    Some(Me { json }) => show_me(json),
                    Some(Show { json }) => show_tasks(json),
                    Some(Add { task }) => add_task(task),
                    Some(Complete { id }) => complete_task(id),
                    Some(Reopen { id }) => reopen_task(id),
                    Some(Delete { id }) => delete_task(id),
                    _ => {
                        println!("command is {:?}", command);
                        return Ok(());
                    }
                }?;
            }
            Err(_) => {}
        }
    }

    //Ok(())
}
