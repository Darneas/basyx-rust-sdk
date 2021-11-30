// SPDX-FileCopyrightText: 2021 Andreas Schmidt <andreas.schmidt@iese.fraunhofer.de>
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdministrativeInformation {
    pub version: Option<String>,
    pub revision: Option<String>,
}

impl AdministrativeInformation {
    pub fn new(version: Option<String>, revision: Option<String>) -> Self {
        Self { version, revision }
    }
}
