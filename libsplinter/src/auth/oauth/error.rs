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

//! Errors that can occur when using OAuth2

use std::error::Error;
use std::fmt;

/// An unrecoverable error that can occur when using an OAuth client
#[derive(Debug)]
pub struct OAuthClientError {
    message: String,
}

impl OAuthClientError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OAuthClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OAuth client encountered an unrecoverable error: {}",
            self.message,
        )
    }
}

impl Error for OAuthClientError {}

/// An error that can occur when configuring an OAuth client
#[derive(Debug)]
pub enum OAuthClientConfigurationError {
    /// The specified authorization URL for the provider was invalid
    InvalidAuthUrl(String),
    /// The specified redirect URL for the client was invalid
    InvalidRedirectUrl(String),
    /// The specified token URL for the provider was invalid
    InvalidTokenUrl(String),
}

impl fmt::Display for OAuthClientConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidAuthUrl(msg) => {
                write!(f, "provided authorization URL is invalid: {}", msg)
            }
            Self::InvalidRedirectUrl(msg) => write!(f, "client redirect URL is invalid: {}", msg),
            Self::InvalidTokenUrl(msg) => write!(f, "provided token URL is invalid: {}", msg),
        }
    }
}

impl Error for OAuthClientConfigurationError {}
