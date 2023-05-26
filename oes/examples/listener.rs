// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use oes::{messages::*, Connector, StackConfiguration};
use std::fs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("template.yaml").expect(&format!("ERROR:: Unable to read file"));

    let adapter_config = serde_yaml::from_str::<StackConfiguration>(&contents).unwrap();

    let connector = Connector::connect(&adapter_config).await.unwrap();

    let mut buf = [0u8; 2048];

    while let Ok((len, _addr)) = connector.sock.recv_from(&mut buf).await {
        let msg = std::str::from_utf8(&buf).unwrap();

        println!("{}", msg);

        match parse_message(&buf, len) {
            Ok(data) => println!("{}", data),
            Err(e) => print!("{}", e),
        }
    }

    Ok(())
}
