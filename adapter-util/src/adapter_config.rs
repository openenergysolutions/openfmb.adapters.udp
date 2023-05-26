#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, usize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdapterConfig {
    #[serde(rename = "plugins")]
    pub plugins: Option<PlugIns>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlugIns {
    #[serde(rename = "oes-plug")]
    pub client: Option<OESPlugin>,

    #[serde(rename = "zenoh")]
    pub zenoh: Option<ZenohPlugin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OESPlugin {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "session")]
    pub sessions: Option<Vec<Session>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenohTopic {
    #[serde(rename = "profile")]
    pub profile: String,

    #[serde(rename = "subject")]
    pub subject: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenohPlugin {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "publish")]
    pub publish: Option<Vec<ZenohTopic>>,

    #[serde(rename = "subscribe")]
    pub subscribe: Option<Vec<ZenohTopic>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecuritySettings {
    #[serde(rename = "security-type")]
    pub security_type: String,

    #[serde(rename = "ca-trusted-cert-file")]
    pub ca_trusted_cert_file: Option<String>,

    #[serde(rename = "client-private-key-file")]
    pub client_private_key_file: Option<String>,

    #[serde(rename = "client-cert-chain-file")]
    pub client_cert_chain_file: Option<String>,

    #[serde(rename = "password")]
    pub password: Option<String>,

    #[serde(rename = "username")]
    pub user_name: Option<String>,

    #[serde(rename = "jwt-creds-file")]
    pub jwt_creds_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandPriorityMap {
    priority_map: HashMap<String, usize>,
}

impl CommandPriorityMap {
    pub fn new(command_order: Option<Vec<String>>) -> CommandPriorityMap {
        let mut cmd = CommandPriorityMap {
            priority_map: HashMap::new(),
        };

        if let Some(v) = command_order {
            for (pos, e) in v.iter().enumerate() {
                cmd.priority_map.insert(e.clone(), pos);
            }
        }
        cmd
    }

    pub fn get_priority(&mut self, command_id: &str) -> usize {
        if let Some(i) = self.priority_map.get(command_id) {
            return *i;
        }
        65535
    }
}
