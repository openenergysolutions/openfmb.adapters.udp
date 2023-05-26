// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{OESPlug, StackConfiguration};

use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

/// OES plug connector
#[derive(Debug, Clone)]
pub struct Connector {
    pub uncontrollable_plugs: Vec<OESPlug>,
    pub controllable_plugs: Vec<OESPlug>,
    pub sock: Arc<UdpSocket>,
}

impl Connector {
    pub async fn connect(
        config: &StackConfiguration,
    ) -> Result<Connector, Box<dyn std::error::Error>> {
        let address = format!("{}:{}", config.ip_address.clone(), config.port);
        let sock = UdpSocket::bind(address.parse::<SocketAddr>()?).await?;
        let sock = Arc::new(sock);

        let connector = Connector {
            uncontrollable_plugs: config.uncontrollable_plugs.clone(),
            controllable_plugs: config.controllable_plugs.clone(),
            sock: sock,
        };

        Ok(connector)
    }

    pub async fn bind_any() -> Option<Arc<UdpSocket>> {
        match UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await {
            Ok(sock) => Some(Arc::new(sock)),
            Err(_e) => None,
        }
    }
}
