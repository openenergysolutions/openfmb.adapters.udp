// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::{publish_profile, subscribe_profile};
use futures::stream::StreamExt;
use oes::connector::Connector as OESConnector;
use oes::{
    Profile, StackConfiguration, OES_PLUG_CURRENT, OES_PLUG_POWER, OES_PLUG_STATUS,
    OES_PLUG_VOLTAGE,
};

use log::error;
use openfmb::bus::{Publisher, Subscriber};

use openfmb_messages_ext::OpenFMBExt;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SwitchProcessor {
    pub bus: Bus,
    pub connector: OESConnector,
    pub adapter_config: AdapterConfig,
    pub stack_config: StackConfiguration,
}

impl SwitchProcessor {
    /// Process indication (reading/status).  We have to process all profiles here because UDP package is broadcasted to a specific port
    pub async fn process_indication(&mut self) {
        // Handle reading and status messages
        let (tx, mut rx) = mpsc::channel(100);

        let runtime = Runtime::new().unwrap();

        let sock = self.connector.sock.clone();

        // Spawn a thread to listen to the socket and send data via channel
        runtime.spawn(async move {
            // Subscribe to UDP messages and publish OpenFMB messages
            let mut buf = [0u8; 1024];
            while let Ok((len, _addr)) = sock.recv_from(&mut buf).await {
                let msg = std::str::from_utf8(&buf).unwrap();

                log::debug!("RECEIVED: {} ({})", msg, len);

                match parse_message(&buf, len) {
                    Ok(data) => {
                        // Look up mrid from MAC address
                        match tx.send(data).await {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("{}", e);
                            }
                        }
                    }
                    Err(e) => print!("{}", e),
                }
            }
        });

        let stack_config = self.stack_config.clone();
        let adapter_config = self.adapter_config.clone();
        let builder = Builder {};
        loop {
            // Upon receiving UDP message, publish OpenFMB messages
            if let Some(data) = rx.recv().await {
                if let Some(id) = stack_config.lookup_mrid_for_uncontrollable(&data.mac_address) {
                    for profile in &stack_config.profiles {
                        match builder.build(&profile.name, &profile.content) {
                            VisitorType::SwitchReading(_s, mut p, mut visitor) => {
                                match visitor.device_mrid() {
                                    Some(device_mrid) => {
                                        // Make sure the publish topic is configured
                                        if zenoh_publishing_topics_enabled(
                                            &adapter_config,
                                            &profile.name,
                                            &device_mrid,
                                        )
                                        .is_some()
                                        {
                                            if device_mrid == id {
                                                // found and matched mRID
                                                log::debug!(
                                                    "Found mRID {} from MAC {}",
                                                    id,
                                                    &data.mac_address
                                                );
                                                visitor.visit(&mut p);

                                                visitor.update_f64(
                                                    OES_PLUG_POWER,
                                                    &mut p,
                                                    data.power,
                                                );
                                                visitor.update_f64(
                                                    OES_PLUG_VOLTAGE,
                                                    &mut p,
                                                    data.voltage,
                                                );
                                                visitor.update_f64(
                                                    OES_PLUG_CURRENT,
                                                    &mut p,
                                                    data.current,
                                                );

                                                publish_profile!(self, &profile.name, p);
                                            }
                                        } else {
                                            log::info!("Publish topic {}.{} is not configured.  Check main adapter configuration file.", profile.name, device_mrid);
                                        }
                                    }
                                    None => {
                                        log::error!(
                                            "Missing device MRID in config file for {}",
                                            &profile.name
                                        )
                                    }
                                }
                            }
                            VisitorType::SwitchStatus(_s, mut p, mut visitor) => {
                                match visitor.device_mrid() {
                                    Some(device_mrid) => {
                                        // Make sure the publish topic is configured
                                        if zenoh_publishing_topics_enabled(
                                            &adapter_config,
                                            &profile.name,
                                            &device_mrid,
                                        )
                                        .is_some()
                                        {
                                            if device_mrid == id {
                                                // found and matched mRID
                                                log::debug!(
                                                    "Found mRID {} from MAC {}",
                                                    id,
                                                    &data.mac_address
                                                );
                                                visitor.visit(&mut p);
                                                let status = data.status.clone();
                                                let status = (status as usize).to_string();

                                                visitor.update_string(
                                                    OES_PLUG_STATUS,
                                                    &mut p,
                                                    status,
                                                );

                                                publish_profile!(self, &profile.name, p);
                                            }
                                        } else {
                                            log::info!("Publish topic {}.{} is not configured.  Check main adapter configuration file.", profile.name, device_mrid);
                                        }
                                    }
                                    None => {
                                        log::error!(
                                            "Missing device MRID in config file for {}",
                                            &profile.name
                                        )
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    /// Process control
    pub async fn process_control(&mut self, profile: Profile) {
        let builder = Builder {};
        let runtime = Runtime::new().unwrap();
        let myself = self.clone();

        runtime.block_on(async move {
            match builder.build(&profile.name, &profile.content) {
                VisitorType::SwitchDiscreteControl(_s, _p, mut visitor) => {
                    match visitor.device_mrid() {
                        Some(device_mrid) => {
                            // Make sure the subscription topic is configured
                            if zenoh_subscribing_topics_enabled(
                                    &myself.adapter_config,
                                    &profile.name,
                                    &device_mrid,
                                )
                                .is_some()
                            {
                                // Check against list of "controllable plugs" see if we can control with the mRID
                                match myself.stack_config.lookup_socker_address_for_controllable(&device_mrid){
                                    Some(addr) => {
                                        // Spawn a thread to subscribe to NATS/Zenoh for OpenFMB control messages
                                        let device_mrid = Uuid::parse_str(&device_mrid).unwrap();
                                        let mut subscription = subscribe_profile!(myself, &profile.name, &device_mrid).unwrap();
                                        debug!("Subscribe to {}.{}...", &profile.name, &device_mrid);
                                        while let Some(ctl) = subscription.next().await {
                                            if let Ok(mut ctl) = ctl {
                                                log::debug!("Got SwitchDiscreteControlProfile message: {:?}", ctl);
                                                visitor.visit(&mut ctl);
                                                let results = visitor.execute_commands(&mut ctl);
                                                log::debug!("Commands: {:?}", results);
                                                let tolerance_ms = visitor.get_tolerance_ms();

                                                match to_commands(results, tolerance_ms, addr.clone()) {
                                                    Some(commands) => {
                                                        // Send command to the plug
                                                        log::debug!("Sending {:?}", commands);
                                                        tokio::spawn(async move {
                                                            execute_commands(commands).await;
                                                        });
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        log::warn!("Missing mRID {} in \"controllable-plugs\".  Skip all controls.", &device_mrid);
                                    }
                                }
                            }
                            else {
                                log::info!("Subcription topic {}.{} is not configured.  Check main adapter configuration file.", profile.name, device_mrid);
                            }
                        }
                        None => {
                            log::error!(
                                "Missing device MRID in config file for {}",
                                &profile.name
                            );
                        }
                    }
                }
                _ => {
                    error!("Only SwitchDiscreteControlProfile is supported.");
                }
            }
        });
    }
}

/// Create profile topic
fn topic(typ: &str, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(
        Module::SwitchModule,
        openfmb::topic::Profile::from_str(typ).unwrap(),
        mrid.clone(),
    )
}
