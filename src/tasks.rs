// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Result};

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

pub fn login() -> Result<()> {
  println!("Loggin In");
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
