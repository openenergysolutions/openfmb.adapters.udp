// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StackConfiguration {
    #[serde(rename = "adapter-ip-address")]
    pub ip_address: String,

    #[serde(rename = "adapter-port")]
    pub port: u32,

    #[serde(rename = "uncontrollable-plugs")]
    pub uncontrollable_plugs: Vec<OESPlug>,

    #[serde(rename = "controllable-plugs")]
    pub controllable_plugs: Vec<OESPlug>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub profiles: Vec<Profile>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OESPlug {
    #[serde(rename = "ip-address")]
    pub ip_address: String,

    #[serde(rename = "port")]
    pub port: u32,

    #[serde(rename = "mac-address")]
    pub mac_address: String,

    #[serde(rename = "mrid")]
    pub mrid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Profile {
    pub name: String,
    pub content: String,
}

impl StackConfiguration {
    /// Look up mRID for a device from its MAC address
    pub fn lookup_mrid_for_uncontrollable(&self, mac_address: &str) -> Option<String> {
        for p in &self.uncontrollable_plugs {
            if p.mac_address == mac_address {
                return Some(p.mrid.clone());
            }
        }
        None
    }

    /// Look up socker address for a device from its mRID
    pub fn lookup_socker_address_for_controllable(&self, mrid: &str) -> Option<SocketAddr> {
        for p in &self.controllable_plugs {
            if p.mrid == mrid {
                match format!("{}:{}", p.ip_address, p.port).parse::<SocketAddr>() {
                    Ok(addr) => return Some(addr),
                    Err(e) => {
                        log::error!("Failed to parse address: {}", e);
                        return None;
                    }
                }
            }
        }
        None
    }
}
