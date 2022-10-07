// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use cli_table::{print_stdout, Table, WithTitle};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::Result;

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

#[derive(Debug, Serialize, Deserialize, Table)]
#[serde(rename_all = "camelCase")]
struct TodoListIdCache {
    #[table(title = "ID")]
    easy_id: String,
    #[table(title = "Display Name")]
    display_name: String,
    #[table(title = "Long ID")]
    id: String,
}

pub fn get_todo_lists(output_format: &str) -> Result<()> {
    let token = read_access_token();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let _res = rt.block_on(async { get_todo_lists_async(output_format, token).await });

    Ok(())
}

async fn get_todo_lists_async(output_format: &str, token: String) -> Result<()> {
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
    }

    match output_format {
        "json" => as_json(&todo_list_id_cache),
        "table" => as_table(&todo_list_id_cache),
        "lines" => as_lines(&todo_list_id_cache),
        _ => as_lines(&todo_list_id_cache),
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

fn as_table(todo_list_id_cache: &Vec<TodoListIdCache>) {
    print_stdout(todo_list_id_cache.with_title()).unwrap();
}

fn as_json(todo_list_id_cache: &Vec<TodoListIdCache>) {
    let json = serde_json::to_string(&todo_list_id_cache);
    println!("{}", json.unwrap());
}

fn as_lines(todo_list_id_cache: &Vec<TodoListIdCache>) {
    for todo_list in todo_list_id_cache {
        println!("[{}] {}", todo_list.easy_id, todo_list.display_name);
    }
}

fn get_config_dir() -> String {
    let proj_dirs = ProjectDirs::from("com", "microsofthackathons", "tdi");
    let config_dir = proj_dirs.unwrap().config_dir().to_path_buf();
    config_dir.into_os_string().into_string().unwrap()
}
