// Copyright 2020 Cargill Incorporated
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

use reqwest::blocking::Client;
use serde::Deserialize;
use splinter::registry::Node;
use std::collections::HashMap;

use crate::action::api::{ServerError, SplinterRestClient};
use crate::error::CliError;

impl SplinterRestClient {
    /// Adds a new node to the registry.
    pub fn add_node(&self, node: &Node) -> Result<(), CliError> {
        // Allowing unused_mut because request must be mutable if experimental feature
        // splinter-cli-jwt is enabled, if feature is removed unused_mut notation can be removed
        #[allow(unused_mut)]
        let mut request = Client::new()
            .post(&format!("{}/registry/nodes", self.url))
            .json(&node);

        #[cfg(feature = "splinter-cli-jwt")]
        {
            request = request.header("Authorization", &self.auth);
        }

        request
            .send()
            .map_err(|err| CliError::ActionError(format!("Failed to add node to registry: {}", err)))
            .and_then(|res| {
                let status = res.status();
                if status.is_success() {
                    Ok(())
                } else {
                    let message = res
                        .json::<ServerError>()
                        .map_err(|_| {
                            CliError::ActionError(format!(
                                "Registry add node request failed with status code '{}', but error response was not valid",
                                status
                            ))
                        })?
                        .message;

                    Err(CliError::ActionError(format!(
                        "Failed to add node to registry: {}",
                        message
                    )))
                }
            })
    }

    /// Updates the node with the same identity in the registry.
    pub fn update_node(&self, node: &Node) -> Result<(), CliError> {
        // Allowing unused_mut because request must be mutable if experimental feature
        // splinter-cli-jwt is enabled, if feature is removed unused_mut notation can be removed
        #[allow(unused_mut)]
        let mut request = Client::new()
            .put(&format!("{}/registry/nodes/{}", self.url, node.identity))
            .json(&node);

        #[cfg(feature = "splinter-cli-jwt")]
        {
            request = request.header("Authorization", &self.auth);
        }

        request
            .send()
            .map_err(|err| CliError::ActionError(format!("Failed to add node to registry: {}", err)))
            .and_then(|res| {
                let status = res.status();
                if status.is_success() {
                    Ok(())
                } else {
                    let message = res
                        .json::<ServerError>()
                        .map_err(|_| {
                            CliError::ActionError(format!(
                                "Registry add node request failed with status code '{}', but error response was not valid",
                                status
                            ))
                        })?
                        .message;

                    Err(CliError::ActionError(format!(
                        "Failed to add node to registry: {}",
                        message
                    )))
                }
            })
    }

    /// Retrieves the node with the given identity from the registry.
    pub fn get_node(&self, identity: &str) -> Result<Option<NodeResponse>, CliError> {
        // Allowing unused_mut because request must be mutable if experimental feature
        // splinter-cli-jwt is enabled, if feature is removed unused_mut notation can be removed
        #[allow(unused_mut)]
        let mut request = Client::new().get(&format!("{}/registry/nodes/{}", self.url, &identity));

        #[cfg(feature = "splinter-cli-jwt")]
        {
            request = request.header("Authorization", &self.auth);
        }

        request.send()
            .map_err(|err| CliError::ActionError(format!("Failed to fetch node: {}", err)))
            .and_then(|res| {
                let status = res.status();
                if status.is_success() {
                    res.json::<NodeResponse>().map(Some).map_err(|_| {
                        CliError::ActionError(
                            "Request was successful, but received an invalid response".into(),
                        )
                    })
                } else {
                    let message = res
                        .json::<ServerError>()
                        .map_err(|_| {
                            CliError::ActionError(format!(
                                "Registry get node request failed with status code '{}', but error response was not valid",
                                status
                            ))
                        })?
                        .message;

                    Err(CliError::ActionError(format!(
                        "Failed to fetch node: {}",
                        message
                    )))
                }
            })
    }
}

#[derive(Debug, Deserialize)]
pub struct NodeResponse {
    pub identity: String,
    pub endpoints: Vec<String>,
    pub display_name: Option<String>,
    pub keys: Vec<String>,
    pub metadata: HashMap<String, String>,
}
