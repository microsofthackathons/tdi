// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

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
    for todo_list in &todo_lists_response.value {
        list_counter += 1;
        println!("[{}] {}", list_counter, &todo_list.display_name);
        //println!("{}", &todo_list.display_name);
    }

    Ok(())
}
