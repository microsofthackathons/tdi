// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Login to Microsoft's Graph database
    Login {},
    /// Logout of Microsoft's SSO
    Logout {},
    /// Display the kanban board.
    Show {
        /// Display as JSON instead of the default table
        json: bool,
    },
    /// Create a new task in ToDo.
    Add {
        /// The task description text.
        task: String,
    },
    /// Complete the given task.
    Complete { id: u32 },
    /// Reopen the given task
    Reopen { id: u32 },
    /// Delete the given task
    Delete { id: u32 },
    /// Show the user's details
    Me {
        /// Display output as "lines", "json" or "table"
        #[clap(default_value = "lines", short, long)]
        output_format: String,
    },
    /// Show the user's To Do lists
    Lists {
        /// Display output as "lines", "json" or "table"
        #[clap(default_value = "lines", short, long)]
        output_format: String,
    },
    Tasks {
        /// Display output as "lines", "json" or "table"
        #[clap(default_value = "lines", short, long)]
        output_format: String,
        /// Display all tasks, including those completed
        #[clap(short, long)]
        display_all: bool,
        list_id: u16,
    },
    /// Repl todo shell
    Intr,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
/// Simple CLI for Microsoft's To Do tasks
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}
