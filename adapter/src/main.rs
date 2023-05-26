// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

mod processors;

use adapter_util::*;
use log::{error, info};
use oes::{Connector, Profile, StackConfiguration};
use processors::*;
use std::{env, fs};

use ctrlc;
use std::sync::mpsc::channel;

const STACK_SIZE: usize = 16 * 1024 * 1024;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // read args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./adapter -c <adapter-config.yaml>");
        return;
    }

    let flag = &args[1];

    if flag != "-c" {
        println!("Usage: ./adapter -c <adapter-config.yaml>");
        return;
    }

    let contents =
        fs::read_to_string(&args[2]).expect(&format!("ERROR:: Unable to read file at {}", args[2]));

    let adapter_config = serde_yaml::from_str::<AdapterConfig>(&contents).unwrap();

    let zenoh = match zenoh_enabled(&adapter_config) {
        Some(b) => {
            if b {
                Some(openfmb::bus::ZenohBus::<openfmb::encoding::ProtobufEncoding>::new())
            } else {
                None
            }
        }
        None => None,
    };

    if zenoh.is_none() {
        // zenoh is not enabled
        panic!("Zenoh is not enabled.  Enable Zenoth in configuration file.");
    }

    match adapter_config.plugins.as_ref().unwrap().client.as_ref() {
        Some(plugin) => {
            if plugin.enabled {
                info!("Initialize UDP Adapter...");
                for session in plugin.sessions.as_ref().unwrap() {
                    let path = session.path.as_ref().unwrap();
                    let contents = fs::read_to_string(path)
                        .expect(&format!("ERROR:: Unable to read file at {}", path));

                    let mut stack_config =
                        serde_yaml::from_str::<StackConfiguration>(&contents).unwrap();

                    let yaml = serde_yaml::from_str::<serde_yaml::Value>(&contents).unwrap();

                    // Create shared connector
                    let connector = Connector::connect(&stack_config).await.unwrap();

                    match &yaml["profiles"] {
                        serde_yaml::Value::Sequence(profiles) => {
                            for p in profiles {
                                //let stack_config = stack_config.clone();
                                //let adapter_config = adapter_config.clone();

                                let profile_name = p
                                    .get(&serde_yaml::Value::String("name".to_string()))
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string();

                                let profile_yaml_string = serde_yaml::to_string(&p).unwrap();

                                if profile_name == "SwitchDiscreteControlProfile" {
                                    let zenoh_bus = zenoh.clone();
                                    let ctor = connector.clone();

                                    let stack_config = stack_config.clone();
                                    let adapter_config = adapter_config.clone();

                                    // process control
                                    let _t = std::thread::Builder::new()
                                        .stack_size(STACK_SIZE)
                                        .spawn(move || {
                                            futures::executor::block_on(process_switch_control(
                                                adapter_config,
                                                stack_config,
                                                Bus {
                                                    zenoh_bus: zenoh_bus,
                                                },
                                                ctor,
                                                Profile {
                                                    name: profile_name,
                                                    content: profile_yaml_string,
                                                },
                                            ));
                                        });
                                } else {
                                    stack_config.profiles.push(Profile {
                                        name: profile_name.clone(),
                                        content: profile_yaml_string.clone(),
                                    });
                                }
                            }
                        }
                        _ => {
                            error!("Unable to parse profiles section in template file.");
                        }
                    }

                    // process
                    let adapter_config = adapter_config.clone();
                    let ctor = connector.clone();
                    let stack_config = stack_config.clone();

                    let zenoh_bus = zenoh.clone();

                    let _t = std::thread::Builder::new()
                        .stack_size(STACK_SIZE)
                        .spawn(move || {
                            futures::executor::block_on(process_switch_indication(
                                adapter_config,
                                stack_config.clone(),
                                Bus {
                                    zenoh_bus: zenoh_bus,
                                },
                                ctor,
                            ));
                        });
                }
            }
        }
        None => {
            error!("No JSON plugin section in main configuration");
        }
    }

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    rx.recv().expect("Could not receive from channel.");
}
