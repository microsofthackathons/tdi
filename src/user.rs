// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use graph_rs_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::auth::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    pub display_name: String,
    pub given_name: String,
    pub surname: String,
    pub id: String,
    pub job_title: String,
    pub mail: String,
    pub mobile_phone: String,
    pub office_location: String,
    pub user_principal_name: String,
}

impl User {
    pub fn new(
        display_name: String,
        given_name: String,
        surname: String,
        id: String,
        job_title: String,
        mail: String,
        mobile_phone: String,
        office_location: String,
        user_principal_name: String,
    ) -> User {
        User {
            display_name,
            given_name,
            surname,
            id,
            job_title,
            mail,
            mobile_phone,
            office_location,
            user_principal_name,
        }
    }

    pub fn empty() -> User {
        User {
            display_name: "".to_string(),
            given_name: "".to_string(),
            surname: "".to_string(),
            id: "".to_string(),
            job_title: "".to_string(),
            mail: "".to_string(),
            mobile_phone: "".to_string(),
            office_location: "".to_string(),
            user_principal_name: "".to_string(),
        }
    }

    pub fn as_table(self) {
        // output user as table
        let table = vec![
            vec![
                "Display Name".cell().bold(true),
                self.display_name.cell().justify(Justify::Right),
            ],
            vec![
                "Given Name".cell().bold(true),
                self.given_name.cell().justify(Justify::Right),
            ],
            vec![
                "Surname".cell().bold(true),
                self.surname.cell().justify(Justify::Right),
            ],
            vec![
                "ID".cell().bold(true),
                self.id.cell().justify(Justify::Right),
            ],
            vec![
                "Mail".cell().bold(true),
                self.mail.cell().justify(Justify::Right),
            ],
            vec![
                "Mobile Phone".cell().bold(true),
                self.mobile_phone.cell().justify(Justify::Right),
            ],
            vec![
                "Office Location".cell().bold(true),
                self.office_location.cell().justify(Justify::Right),
            ],
            vec![
                "User Principal Name".cell().bold(true),
                self.user_principal_name.cell().justify(Justify::Right),
            ],
        ]
        .table();
        //.title(vec!["Items".cell().bold(true), "Details".cell().bold(true)])
        //.bold(true);

        match print_stdout(table) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }

    pub fn as_json(self) {
        // output user as json
        let json = serde_json::to_string(&self);
        println!("{}", json.unwrap());
    }

    pub fn as_lines(self) {
        println!("Display Name: {:?}", self.display_name);
        println!("Given Name: {:?}", self.given_name);
        println!("Surname: {:?}", self.surname);
        println!("ID: {:?}", self.id);
        println!("Mail: {:?}", self.mail);
        println!("Mobile Phone: {:?}", self.mobile_phone);
        println!("Office Location: {:?}", self.office_location);
        println!("User Principal Name: {:?}", self.user_principal_name);
    }
}

pub fn show_me(output_format: &str) -> Result<()> {
    let token = read_access_token();
    let client = Graph::new(&token);

    match client.v1().me().get_user().send() {
        Ok(res) => {
            let user = match res.body().as_object() {
                None => User::empty(),
                Some(map) => User::new(
                    map.get(&"displayName".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"givenName".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"surname".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"id".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"jobTitle".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"mail".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"mobilePhone".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"officeLocation".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                    map.get(&"userPrincipalName".to_string())
                        .unwrap()
                        .to_string()
                        .replace("\"", ""),
                ),
            };

            match output_format {
                "json" => {
                    user.as_json();
                }
                "lines" => {
                    user.as_lines();
                }
                "table" => {
                    user.as_table();
                }
                _ => {
                    user.as_lines();
                }
            }
        }
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}
