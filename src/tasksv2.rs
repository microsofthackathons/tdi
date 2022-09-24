// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::auth::read_access_token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoTasks {
    importance: String,
    status: String,
    title: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoTaskResponse {
    value: Vec<TodoTasks>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoListIdCache {
    display_name: String,
    id: String,
    easy_id: String,
}

pub fn get_todo_tasks(id: &u16) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let _res = rt.block_on(async { get_todo_tasks_async(id).await });

    Ok(())
}

async fn get_todo_tasks_async(id: &u16) -> Result<()> {
    let token = read_access_token();
    let client = reqwest::Client::new();

    let real_id = get_real_list_id(id);

    let response = client
        .get(format!(
            "https://graph.microsoft.com/v1.0/me/todo/lists/{real_id}/tasks",
            real_id = real_id
        ))
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    //dbg!("response: {:?}", &response);
    let body = response.text().await.unwrap();
    //dbg!("body: {:?}", &body);
    let todo_tasks_response: TodoTaskResponse = serde_json::from_str(&body).unwrap();
    //dbg!("todo_lists: {:?}", &todo_tasks_response);

    let mut list_counter = 0i16;

    for task in &todo_tasks_response.value {
        list_counter += 1;
        println!(
            "[{}] {} {} {}",
            list_counter, &task.title, &task.importance, &task.status
        );
        //println!("{}", &todo_list.display_name);
    }

    Ok(())
}

fn get_config_dir() -> String {
    let proj_dirs = ProjectDirs::from("com", "microsofthackathons", "tdi");
    let config_dir = proj_dirs.unwrap().config_dir().to_path_buf();
    config_dir.into_os_string().into_string().unwrap()
}

fn get_real_list_id(easy_id: &u16) -> String {
    let mut real_id = String::new();
    let todo_list_id_cache: Vec<TodoListIdCache> = serde_json::from_reader(
        std::fs::File::open(get_config_dir() + "/lists_cache.json").unwrap(),
    )
    .unwrap();

    for todo_list in &todo_list_id_cache {
        if todo_list.easy_id == easy_id.to_string() {
            real_id = todo_list.id.clone();
        }
    }

    real_id
}
