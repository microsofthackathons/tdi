// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

mod auth;
mod cli;
mod lists;
mod tasks;
mod tasksv2;
mod user;

use clap::Parser;

use cli::{Cli, Commands::*};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Login {}) => auth::login(),
        Some(Logout {}) => auth::logout(),
        Some(Me { output_format }) => user::show_me(output_format),
        Some(Show { json }) => tasks::show_tasks(json),
        Some(Add { task }) => tasks::add_task(task),
        Some(Complete { id }) => tasks::complete_task(id),
        Some(Reopen { id }) => tasks::reopen_task(id),
        Some(Delete { id }) => tasks::delete_task(id),
        Some(Lists { output_format }) => lists::get_todo_lists(output_format),
        Some(Tasks { list_id }) => tasksv2::get_todo_tasks(list_id),
        Some(Intr) => tasks::interactive(),
        None => {
            println!("Default subcommand");
            Ok(())
        }
    }?;

    Ok(())
}
