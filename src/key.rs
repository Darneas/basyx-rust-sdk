// SPDX-FileCopyrightText: 2021 Andreas Schmidt <andreas.schmidt@iese.fraunhofer.de>
//
// SPDX-License-Identifier: EPL-2.0

use crate::key_type::KeyType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub r#type: String,
    pub value: String,
    pub id_type: KeyType,
    pub local: Option<bool>,
    pub index: Option<i32>,
}

impl Key {
    pub fn new(r#type: String, value: String, id_type: KeyType) -> Self {
        Self {
            r#type,
            local: None,
            value,
            index: None,
            id_type,
        }
    }
}
