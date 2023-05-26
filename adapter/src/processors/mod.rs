// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use adapter_util::*;
use core::str::FromStr;
use log::{debug, error};
use oes::connector::Connector as OESConnector;
use oes::{
    messages::{parse_message, set_relay_message, OES_PLUG_COMMAND},
    StackConfiguration,
};
use openfmb::prelude::*;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;

pub mod switch;
pub use switch::*;

#[derive(Clone, Debug)]
pub struct Commands {
    pub values: Vec<String>,
    pub ts: Option<CommandTimestamp>,
    pub tolerance_ms: Option<u32>,
    pub socket_address: SocketAddr,
}

pub async fn process_switch_indication(
    adapter_config: AdapterConfig,
    stack_config: StackConfiguration,
    bus: Bus,
    connector: OESConnector,
) {
    let mut processor = SwitchProcessor {
        bus: bus,
        connector: connector,
        adapter_config: adapter_config,
        stack_config: stack_config.clone(),
    };

    processor.process_indication().await;
}

pub async fn process_switch_control(
    adapter_config: AdapterConfig,
    stack_config: StackConfiguration,
    bus: Bus,
    connector: OESConnector,
    profile: oes::Profile,
) {
    let mut processor = SwitchProcessor {
        bus: bus,
        connector: connector,
        adapter_config: adapter_config,
        stack_config: stack_config.clone(),
    };

    processor.process_control(profile).await;
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub zenoh_bus: Option<openfmb::bus::ZenohBus<openfmb::encoding::ProtobufEncoding>>,
}

async fn do_send_commands(commands: Commands) {
    match OESConnector::bind_any().await {
        Some(sock) => {
            match sock.connect(commands.socket_address).await {
                Ok(_) => {
                    for c in commands.values {
                        for _n in 1..3 {
                            // for retry
                            match sock.send(c.as_bytes()).await {
                                Ok(_) => {
                                    debug!("Sent {} to {}", &c, &commands.socket_address);
                                    break;
                                }
                                Err(e) => error!(
                                    "Failed to send {} to {} with error: {}",
                                    &c, &commands.socket_address, e
                                ),
                            }
                        }
                    }
                }
                Err(e) => error!("Unable to connecto to {}: {}", commands.socket_address, e),
            }
        }
        None => {
            error!("Unable to bind to local socket");
        }
    }
}

pub async fn execute_commands(commands: Commands) {
    debug!("Command count: {}", commands.values.len());
    match commands.ts {
        Some(_time) => {
            // handle schedule
            schedule(commands).await;
        }
        _ => {
            do_send_commands(commands).await;
        }
    }
}

async fn schedule(commands: Commands) {
    let now = get_current_timestamp().seconds;
    let mut ok = true;
    let seconds = commands.ts.unwrap().seconds;
    if seconds > now {
        sleep(Duration::from_secs(seconds - now)).await;
    } else {
        let tolerance: u64 = match commands.tolerance_ms {
            Some(ms) => ms as u64,
            _ => 0,
        };
        if (now - seconds) * 1000 < tolerance {
            ok = false;
            error!(
                "The schedule time is not within the allowed tolerance: {} (tolerance: {})",
                seconds, tolerance
            );
        }
    }

    if ok {
        do_send_commands(commands).await;
    }
}

pub fn zenoh_enabled(adapter_config: &AdapterConfig) -> Option<bool> {
    Some(adapter_config.plugins.as_ref()?.zenoh.as_ref()?.enabled)
}

pub fn zenoh_publishing_topics_enabled(
    adapter_config: &AdapterConfig,
    profile: &str,
    mrid: &str,
) -> Option<bool> {
    match zenoh_enabled(adapter_config) {
        Some(b) => {
            if !b {
                return Some(false);
            }
        }
        None => {
            return None;
        }
    }

    let list: Vec<ZenohTopic> = match adapter_config
        .plugins
        .as_ref()?
        .zenoh
        .as_ref()?
        .publish
        .as_ref()
    {
        Some(v) => v.to_vec(),
        _ => {
            vec![]
        }
    };

    for v in list {
        if v.profile == profile {
            if &v.subject == "*" || &v.subject == mrid {
                debug!("Zenoh publication to {}.{} is enabled.", profile, mrid);
                return Some(true);
            }
        }
    }
    error!("Zenoh publication to {}.{} is disabled.", profile, mrid);
    None
}

pub fn zenoh_subscribing_topics_enabled(
    adapter_config: &AdapterConfig,
    profile: &str,
    mrid: &str,
) -> Option<bool> {
    match zenoh_enabled(adapter_config) {
        Some(b) => {
            if !b {
                return Some(false);
            }
        }
        None => {
            return None;
        }
    }

    let list: Vec<ZenohTopic> = match adapter_config
        .plugins
        .as_ref()?
        .zenoh
        .as_ref()?
        .subscribe
        .as_ref()
    {
        Some(v) => v.to_vec(),
        _ => {
            vec![]
        }
    };

    for v in list {
        if v.profile == profile {
            if &v.subject == "*" || &v.subject == mrid {
                debug!("Zenoh subscription to {}.{} is enabled.", profile, mrid);
                return Some(true);
            }
        }
    }
    error!("Zenoh subscription to {}.{} is disabled.", profile, mrid);
    None
}

fn to_commands(
    commands: Vec<Command>,
    tolerance_ms: Option<u32>,
    socket_address: SocketAddr,
) -> Option<Commands> {
    let mut list: Vec<String> = Vec::new();

    let mut ts: Option<CommandTimestamp> = None;
    for r in commands {
        match r {
            Command::IntValue(_b, v, t) => {
                if t.is_some() {
                    ts = t;
                }

                for c in v {
                    match c.name.as_str() {
                        OES_PLUG_COMMAND => {
                            if let Some(val) = c.real_value {
                                if val > 0.0 {
                                    list.push(set_relay_message(true));
                                } else {
                                    list.push(set_relay_message(false));
                                }
                            }
                        }
                        _ => {
                            log::warn!("Command with name {} is not supported", c.name);
                        }
                    }
                }
            }
            Command::BoolValue(_b, v, t) => {
                if t.is_some() {
                    ts = t;
                }

                for c in v {
                    match c.name.as_str() {
                        OES_PLUG_COMMAND => {
                            if let Some(val) = c.bool_value {
                                if val {
                                    list.push(set_relay_message(true));
                                } else {
                                    list.push(set_relay_message(false));
                                }
                            }
                        }
                        _ => {
                            log::warn!("Command with name {} is not supported", c.name);
                        }
                    }
                }
            }
            _ => {
                log::warn!("Command not supported: {:?}", r);
            }
        }
    }

    match list.len() > 0 {
        false => None,
        true => Some(Commands {
            values: list,
            ts,
            tolerance_ms,
            socket_address,
        }),
    }
}

#[macro_export]
macro_rules! publish_profile {
    ($processor:expr, $profile_name:expr, $msg:expr) => {
        match $processor
            .bus
            .zenoh_bus
            .clone()
            .unwrap()
            .publish(
                topic($profile_name, &$msg.device_mrid().unwrap()).iter(),
                $msg,
            )
            .await
        {
            Ok(_) => {
                debug!("{} published!", $profile_name);
            }
            Err(e) => {
                error!("Zenoh::Failed to publish message: {}", e)
            }
        }
    };
}

#[macro_export]
macro_rules! subscribe_profile {
    ($processor:expr, $profile_name:expr, $mrid:expr) => {{
        $processor
            .bus
            .zenoh_bus
            .clone()
            .unwrap()
            .subscribe(topic($profile_name, $mrid).iter())
            .await
    }};
}
