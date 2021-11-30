// SPDX-FileCopyrightText: 2021 Andreas Schmidt <andreas.schmidt@iese.fraunhofer.de>
//
// SPDX-License-Identifier: EPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AssetKind {
    Type,
    Instance,
}
