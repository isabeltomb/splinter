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

//! Provides structures for the REST resources.

#[cfg(any(feature = "biome-key-management", feature = "biome-credentials"))]
pub(in crate::biome::rest_api) mod authorize;
#[cfg(feature = "biome-credentials")]
pub(in crate::biome::rest_api) mod credentials;
#[cfg(feature = "biome-key-management")]
pub(in crate::biome::rest_api) mod key_management;
#[cfg(feature = "biome-refresh-tokens")]
pub(in crate::biome::rest_api) mod token;
