// Copyright 2018-2020 Cargill Incorporated
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

#[cfg(feature = "registry-add-node")]
mod api;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::ArgMatches;
use splinter::registry::Node;
#[cfg(feature = "registry-add-node")]
use std::collections::HashMap;

use crate::error::CliError;

use super::api::SplinterRestClientBuilder;
use super::{
    msg_from_io_error, read_private_key, Action, DEFAULT_SPLINTER_REST_API_URL,
    SPLINTER_REST_API_URL_ENV,
};

#[cfg(feature = "splinter-cli-jwt")]
use super::create_cylinder_jwt_auth;

const DEFAULT_OUTPUT_FILE: &str = "./nodes.yaml";

pub struct RegistryGenerateAction;

impl Action for RegistryGenerateAction {
    fn run<'a>(&mut self, arg_matches: Option<&ArgMatches<'a>>) -> Result<(), CliError> {
        let args = arg_matches.ok_or(CliError::RequiresArgs)?;

        let output_file = args.value_of("file").unwrap_or(DEFAULT_OUTPUT_FILE);

        let mut nodes: Vec<Node> = if Path::new(output_file).exists() {
            let file = File::open(output_file).map_err(|err| {
                CliError::ActionError(format!(
                    "Failed to open '{}': {}",
                    output_file,
                    msg_from_io_error(err)
                ))
            })?;
            serde_yaml::from_reader(file).map_err(|_| {
                CliError::ActionError(format!(
                    "Failed to read registry file '{}': Not a valid YAML sequence of nodes",
                    output_file
                ))
            })?
        } else {
            vec![]
        };

        let url = args
            .value_of("status_url")
            .map(ToOwned::to_owned)
            .or_else(|| std::env::var(SPLINTER_REST_API_URL_ENV).ok())
            .unwrap_or_else(|| DEFAULT_SPLINTER_REST_API_URL.to_string());

        let mut builder = SplinterRestClientBuilder::new();
        builder = builder.with_url(url);

        #[cfg(feature = "splinter-cli-jwt")]
        {
            let key = arg_matches.and_then(|args| args.value_of("private_key_file"));
            builder = builder.with_auth(create_cylinder_jwt_auth(key)?);
        }

        let client = builder.build()?;

        let node_status = client.get_node_status()?;

        let keys = args
            .values_of("key_files")
            .ok_or_else(|| CliError::ActionError("One or more key files must be specified".into()))?
            .map(|key_file| read_private_key(key_file))
            .collect::<Result<_, _>>()?;

        let metadata = if let Some(metadata) = args.values_of("metadata") {
            metadata
                .map(|kv| {
                    let mut kv_iter = kv.splitn(2, '=');

                    let key = kv_iter
                        .next()
                        .expect("str::split cannot return an empty iterator")
                        .to_string();
                    if key.is_empty() {
                        return Err(CliError::ActionError(
                            "Empty '--metadata' argument detected".into(),
                        ));
                    }

                    let value = kv_iter
                        .next()
                        .ok_or_else(|| {
                            CliError::ActionError(format!(
                                "Missing value for metadata key '{}'",
                                key
                            ))
                        })?
                        .to_string();
                    if value.is_empty() {
                        return Err(CliError::ActionError(format!(
                            "Empty value detected for metadata key '{}'",
                            key
                        )));
                    }

                    Ok((key, value))
                })
                .collect::<Result<_, _>>()?
        } else {
            Default::default()
        };

        let node = Node {
            identity: node_status.node_id.clone(),
            endpoints: node_status.advertised_endpoints,
            display_name: node_status.display_name,
            keys,
            metadata,
        };

        if let Some(idx) = nodes
            .iter()
            .position(|existing_node| existing_node.identity == node.identity)
        {
            if args.is_present("force") {
                nodes.remove(idx);
            } else {
                return Err(CliError::EnvironmentError(format!(
                    "Node '{}' already exists; must use '--force' to overwrite an existing node",
                    node.identity
                )));
            }
        }

        nodes.push(node);

        let yaml = serde_yaml::to_vec(&nodes).map_err(|err| {
            CliError::ActionError(format!("Cannot format node list into yaml: {}", err))
        })?;

        let mut file = File::create(output_file).map_err(|err| {
            CliError::ActionError(format!(
                "Failed to create or overwrite '{}': {}",
                output_file,
                msg_from_io_error(err)
            ))
        })?;
        file.write_all(&yaml).map_err(|err| {
            CliError::ActionError(format!(
                "Failed to write to file '{}': {}",
                output_file,
                msg_from_io_error(err)
            ))
        })?;
        // Append newline to file
        writeln!(file).map_err(|err| {
            CliError::ActionError(format!(
                "Failed to write to file '{}': {}",
                output_file,
                msg_from_io_error(err)
            ))
        })?;

        info!("Added node '{}' to '{}'", node_status.node_id, output_file);

        Ok(())
    }
}

#[cfg(feature = "registry-add-node")]
pub struct RegistryAddAction;

#[cfg(feature = "registry-add-node")]
impl Action for RegistryAddAction {
    fn run<'a>(&mut self, arg_matches: Option<&ArgMatches<'a>>) -> Result<(), CliError> {
        let args = arg_matches.ok_or(CliError::RequiresArgs)?;

        let url = args
            .value_of("url")
            .map(ToOwned::to_owned)
            .or_else(|| std::env::var(SPLINTER_REST_API_URL_ENV).ok())
            .unwrap_or_else(|| DEFAULT_SPLINTER_REST_API_URL.to_string());

        let identity = args
            .value_of("identity")
            .ok_or_else(|| CliError::ActionError("Identity must be specified".into()))?
            .to_string();

        let display_name = args
            .value_of("display_name")
            .unwrap_or(&identity)
            .to_string();

        let mut builder = SplinterRestClientBuilder::new();
        builder = builder.with_url(url);

        #[cfg(feature = "splinter-cli-jwt")]
        {
            let private_key = args.value_of("private_key_file");
            builder = builder.with_auth(create_cylinder_jwt_auth(private_key)?);
        }

        let client = builder.build()?;

        let metadata: Vec<(String, String)> = if let Some(metadata) = args.values_of("metadata") {
            metadata
                .map(|kv| {
                    let mut kv_iter = kv.splitn(2, '=');

                    let key = kv_iter
                        .next()
                        .expect("str::split cannot return an empty iterator")
                        .to_string();
                    if key.is_empty() {
                        return Err(CliError::ActionError(
                            "Empty '--metadata' argument detected".into(),
                        ));
                    }

                    let value = kv_iter
                        .next()
                        .ok_or_else(|| {
                            CliError::ActionError(format!(
                                "Missing value for metadata key '{}'",
                                key
                            ))
                        })?
                        .to_string();
                    if value.is_empty() {
                        return Err(CliError::ActionError(format!(
                            "Empty value detected for metadata key '{}'",
                            key
                        )));
                    }

                    Ok((key, value))
                })
                .collect::<Result<_, _>>()?
        } else {
            Default::default()
        };

        if args.is_present("from_remote") {
            let remote_node = client.get_node(&identity)?.ok_or_else(|| {
                CliError::ActionError("Unable to retrieve node from remote".into())
            })?;
            let endpoints = match args.values_of("endpoint") {
                Some(endpoints) => endpoints.map(String::from).collect::<Vec<String>>(),
                None => remote_node.endpoints,
            };
            let keys = match args.values_of("key") {
                Some(keys) => keys
                    .map(|key_file| read_private_key(key_file))
                    .collect::<Result<_, _>>()?,
                None => remote_node.keys,
            };

            let mut node_metadata: HashMap<String, String> = HashMap::new();
            if args.is_present("metadata") {
                for (key, value) in metadata.iter() {
                    node_metadata.insert(key.to_string(), value.to_string());
                }
            } else {
                node_metadata = remote_node.metadata;
            }

            let mut node_builder = Node::builder(identity)
                .with_endpoints(endpoints)
                .with_display_name(display_name)
                .with_keys(keys);

            for (key, value) in node_metadata {
                node_builder = node_builder.with_metadata(key, value);
            }

            let node = node_builder
                .build()
                .map_err(|err| CliError::ActionError(format!("Unable to build node: {}", err)))?;

            if !args.is_present("dry_run") {
                client.update_node(&node)?;
            }

            info!("{}", node);

            Ok(())
        } else {
            let endpoints: Vec<String> = args
                .values_of("endpoint")
                .ok_or_else(|| {
                    CliError::ActionError("One or more endpoints must be specified".into())
                })?
                .map(String::from)
                .collect::<Vec<String>>();

            let keys: Vec<String> = args
                .values_of("key_files")
                .ok_or_else(|| {
                    CliError::ActionError("One or more key files must be specified".into())
                })?
                .map(|key_file| read_private_key(key_file))
                .collect::<Result<_, _>>()?;

            let mut node_builder = Node::builder(identity)
                .with_endpoints(endpoints)
                .with_display_name(display_name)
                .with_keys(keys);

            for (key, value) in metadata.iter() {
                node_builder = node_builder.with_metadata(key, value);
            }

            let node = node_builder
                .build()
                .map_err(|err| CliError::ActionError(format!("Unable to build node: {}", err)))?;

            if !args.is_present("dry_run") {
                client.add_node(&node)?
            }

            info!("{}", node);

            Ok(())
        }
    }
}
