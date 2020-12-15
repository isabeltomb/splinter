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

use diesel::prelude::*;

use crate::error::InternalError;

use crate::biome::oauth::store::{
    diesel::{models::OAuthUserModel, schema::oauth_user},
    OAuthUserStoreError,
};

use super::OAuthUserStoreOperations;

pub(in crate::biome::oauth) trait OAuthUserStoreListByProviderUserRef {
    fn list_by_provider_user_ref(
        &self,
        provider_user_ref: &str,
    ) -> Result<Vec<OAuthUserModel>, OAuthUserStoreError>;
}

impl<'a, C> OAuthUserStoreListByProviderUserRef for OAuthUserStoreOperations<'a, C>
where
    C: diesel::Connection,
    i16: diesel::deserialize::FromSql<diesel::sql_types::SmallInt, C::Backend>,
    i64: diesel::deserialize::FromSql<diesel::sql_types::BigInt, C::Backend>,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, C::Backend>,
{
    fn list_by_provider_user_ref(
        &self,
        provider_user_ref: &str,
    ) -> Result<Vec<OAuthUserModel>, OAuthUserStoreError> {
        oauth_user::table
            .into_boxed()
            .select(oauth_user::all_columns)
            .filter(oauth_user::provider_user_ref.eq(provider_user_ref))
            .load::<OAuthUserModel>(self.conn)
            .map_err(|err| {
                OAuthUserStoreError::InternalError(InternalError::from_source(Box::new(err)))
            })
    }
}