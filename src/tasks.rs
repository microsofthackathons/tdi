// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use chrono::{serde::ts_seconds, DateTime, Utc};
use clap::Parser;
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
                println!("Me as a table: {:?}", res.body());
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
