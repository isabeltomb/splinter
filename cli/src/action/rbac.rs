// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::ArgMatches;

use crate::error::CliError;

use super::{
    api::{SplinterRestClient, SplinterRestClientBuilder},
    create_cylinder_jwt_auth, print_table, Action, DEFAULT_SPLINTER_REST_API_URL,
    SPLINTER_REST_API_URL_ENV,
};

pub struct ListRolesAction;

impl Action for ListRolesAction {
    fn run<'a>(&mut self, arg_matches: Option<&ArgMatches<'a>>) -> Result<(), CliError> {
        let format = arg_matches
            .and_then(|args| args.value_of("format"))
            .unwrap_or("human");

        let client = new_client(&arg_matches)?;
        let roles = client.list_roles()?;

        let header = vec!["ID".to_string(), "DISPLAY NAME".to_string()];

        if format == "csv" {
            println!("{}", header.join(","));
            for role_res in roles {
                let role = role_res?;
                println!("{},{}", role.role_id, role.display_name);
            }
        } else {
            let mut rows = vec![header];
            for role_res in roles {
                let role = role_res?;
                rows.push(vec![role.role_id, role.display_name]);
            }
            print_table(rows);
        }

        Ok(())
    }
}

fn new_client(arg_matches: &Option<&ArgMatches<'_>>) -> Result<SplinterRestClient, CliError> {
    let url = arg_matches
        .and_then(|args| args.value_of("url"))
        .map(ToOwned::to_owned)
        .or_else(|| std::env::var(SPLINTER_REST_API_URL_ENV).ok())
        .unwrap_or_else(|| DEFAULT_SPLINTER_REST_API_URL.to_string());

    let key = arg_matches.and_then(|args| args.value_of("private_key_file"));

    SplinterRestClientBuilder::new()
        .with_url(url)
        .with_auth(create_cylinder_jwt_auth(key)?)
        .build()
}