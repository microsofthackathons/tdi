# Basic Design-y Things

## Initial thoughts to guide the project through the 2022 Hackathon

* AuthN is the biggest initial challenge
  
  https://github.com/Azure/azure-sdk-for-rust or https://github.com/sreeise/graph-rs or other?

* CLI before TUI
  
  While I would like a clone of the full GUI app in [tui-rs](https://github.com/fdehau/tui-rs), the initial effort should focus on providing a basic "command" structure, executable from a shell, to access Microsoft To Do stored tasks.  For simplicity we'll start with [basic CLI](https://www.rust-lang.org/what/cli) interfaces and can follow the [book](https://rust-cli.github.io/book/index.html) as a guide. 
  
  Using [clap](https://docs.rs/clap/latest/clap/).

* Options _should_ be passable to the CLI or read from a config file in ~/.config/tdi.{conf|toml} - this is a stretch goal
  
* Each command should be able to return JSON or a table view, for the sake of downstream integration potential - e.g., `$ tdi show -- --json`
  
* Command and subcommands - e.g., `$ tdi add "this is a new task"` and `$ tdi list --important`
  
## Project Todos 

Track these things as issues.  Will take a whack at populating a few things.