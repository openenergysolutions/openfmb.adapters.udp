// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use oes::{set_relay_message, Connector, StackConfiguration};
use std::{fs, io, net::SocketAddr};

#[tokio::main]
async fn main() -> io::Result<()> {
    let contents =
        fs::read_to_string("template.yaml").expect(&format!("ERROR:: Unable to read file"));

    let adapter_config = serde_yaml::from_str::<StackConfiguration>(&contents).unwrap();

    let connector = Connector::connect(&adapter_config).await.unwrap();

    for p in adapter_config.controllable_plugs {
        let addr = format!("{}:{}", p.ip_address, p.port);
        let remote_addr = addr.parse::<SocketAddr>().unwrap();

        connector.sock.connect(remote_addr).await?;
        let msg = set_relay_message(false);
        let buf = msg.as_bytes();

        // send to the plug
        let _len = connector.sock.send(buf).await?;

        let mut buf = [0u8; 256];
        // recv from the plug
        let _len = connector.sock.recv(&mut buf).await?;

        println!("Received: {}", std::str::from_utf8(&buf).unwrap());
    }

    Ok(())
}
