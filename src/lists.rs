// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::io::Result;
use directories::ProjectDirs;

use crate::auth::read_access_token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoList {
    display_name: String,
    is_owner: bool,
    is_shared: bool,
    id: String,
    wellknown_list_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoListResponse {
    value: Vec<TodoList>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoListIdCache {
    display_name: String,
    id: String,
    easy_id: String,
}

pub fn get_todo_lists() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let _res = rt.block_on(async { get_todo_lists_async().await });

    Ok(())
}

async fn get_todo_lists_async() -> Result<()> {
    let token = read_access_token();
    let client = reqwest::Client::new();

    let response = client
        .get("https://graph.microsoft.com/v1.0/me/todo/lists")
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    //dbg!("response: {:?}", &response);
    let body = response.text().await.unwrap();
    //dbg!("body: {:?}", &body);
    let todo_lists_response: TodoListResponse = serde_json::from_str(&body).unwrap();
    //dbg!("todo_lists: {:?}", &todo_lists_response);

    let mut list_counter = 0i16;
    let mut todo_list_id_cache: Vec<TodoListIdCache> = Vec::new();

    for todo_list in &todo_lists_response.value {
        list_counter += 1;
        todo_list_id_cache.push(TodoListIdCache {
            display_name: todo_list.display_name.clone(),
            id: todo_list.id.clone(),
            easy_id: list_counter.to_string(),
        });
        println!("[{}] {}", list_counter, &todo_list.display_name);
        //println!("{}", &todo_list.display_name);
    }

    // We need to cache the list ids so we can use them later
    // to use the simpler 0-n ids. The real ones are unmanageable:
    // AQMkADAwATMwMAItYjBkZPPtZWQ0ZS0wWEItMDAKAC4AAANkdZgpr8LxTL4LkrPqypbXAQBPdIWRHCTMQpY9NGnpa9LvAAACARIAAAA=
    let _result = serde_json::to_writer_pretty(
        std::fs::File::create(get_config_dir() + "/lists_cache.json").unwrap(),
        &todo_list_id_cache,
    );

    Ok(())
}

fn get_config_dir() -> String {
    let proj_dirs = ProjectDirs::from("com", "microsofthackathons", "tdi");
    let config_dir = proj_dirs.unwrap().config_dir().to_path_buf();
    config_dir.into_os_string().into_string().unwrap()
}