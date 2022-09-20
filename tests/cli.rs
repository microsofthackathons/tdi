// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use assert_cmd::prelude::*; // Add methods on commands
                            //use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tdi")?;
    cmd.arg("help");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains(
            "Simple CLI for Microsoft's To Do tasks",
        ));
    Ok(())
}
