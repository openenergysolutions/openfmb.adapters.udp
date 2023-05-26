// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub const OES_PLUG_STATUS: &str = "OES.Plug.Status";
pub const OES_PLUG_POWER: &str = "OES.Plug.Power";
pub const OES_PLUG_VOLTAGE: &str = "OES.Plug.Voltage";
pub const OES_PLUG_CURRENT: &str = "OES.Plug.Current";

pub const OES_PLUG_COMMAND: &str = "OES.Plug.Command";

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum PlugStatus {
    Off,
    On,
    Unknown,
}

impl Default for PlugStatus {
    fn default() -> Self {
        PlugStatus::Off
    }
}

impl Display for PlugStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let state = match self {
            PlugStatus::Off => "Off",
            PlugStatus::On => "On",
            PlugStatus::Unknown => "Unknown",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub ip_address: String,
    pub mac_address: String,
    pub name: String,
    pub status: PlugStatus,
    pub power: f64,
    pub voltage: f64,
    pub current: f64,
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "IP: {}; MAC: {}; Name: {}; Status: {}; Power: {}; Voltage: {:?}; Current: {:?}",
            self.ip_address,
            self.mac_address,
            self.name,
            self.status,
            self.power,
            self.voltage,
            self.current,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FC {
    Off,
    On,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    fc: FC,
}

pub fn set_relay_message(on_off: bool) -> String {
    let request = match on_off {
        true => Request { fc: FC::On },
        false => Request { fc: FC::Off },
    };

    serde_json::to_string(&request).unwrap()
}

pub fn parse_message(buf: &[u8], len: usize) -> std::result::Result<Data, &'static str> {
    use std::str::from_utf8;

    let json = from_utf8(&buf[0..len])
        .map_err(|_e| "Error parse message")?
        .trim_end();

    log::debug!("RECEIVED: {}", json);

    let data = serde_json::from_str::<Data>(json).map_err(|_e| {
        log::error!("{}", _e);
        "Error deserializing data\n"
    })?;

    Ok(data)
}
