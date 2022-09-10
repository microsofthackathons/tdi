// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

mod cli;
mod tasks;

use clap::Parser;

use cli::{Commands::*, Cli};





fn main() -> anyhow::Result<()> {
    let cli= Cli::parse();

    match &cli.command {
        Some(Login {}) => tasks::login(),
        Some(Show { json }) => tasks::show_tasks(json),
        Some(Add { task }) => tasks::add_task(task),
        Some(Complete { id }) => tasks::complete_task(id),
        Some(Reopen { id }) => tasks::reopen_task(id),
        Some(Delete { id }) => tasks::delete_task(id),
        None => {
            println!("Default subcommand");
            Ok(())
        }
    }?;

    Ok(())

}