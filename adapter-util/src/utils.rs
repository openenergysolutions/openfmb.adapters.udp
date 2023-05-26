// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use super::configuration::*;
use super::visitors::*;
use log::error;

use openfmb::messages::switchmodule::*;

pub enum VisitorType {
    SwitchDiscreteControl(
        String,
        SwitchDiscreteControlProfile,
        SwitchDiscreteControlProfileVisitor,
    ),
    SwitchReading(String, SwitchReadingProfile, SwitchReadingProfileVisitor),
    SwitchStatus(String, SwitchStatusProfile, SwitchStatusProfileVisitor),
}

pub trait VisitorBuilder {
    fn build(&self, profile_name: &str, profile_config_string: &str) -> VisitorType {
        match profile_name {
            "SwitchDiscreteControlProfile" => {
                match serde_yaml::from_str::<SwitchDiscreteControlProfileMapping>(
                    &profile_config_string,
                ) {
                    Ok(mapping) => VisitorType::SwitchDiscreteControl(
                        "switchmodule".into(),
                        SwitchDiscreteControlProfile::default(),
                        SwitchDiscreteControlProfileVisitor::new(mapping),
                    ),
                    Err(e) => {
                        error!("{}", e);
                        panic!("Unable to parse profile mapping configuration (yaml file)!");
                    }
                }
            }
            "SwitchReadingProfile" => {
                match serde_yaml::from_str::<SwitchReadingProfileMapping>(&profile_config_string) {
                    Ok(mapping) => VisitorType::SwitchReading(
                        "switchmodule".into(),
                        SwitchReadingProfile::default(),
                        SwitchReadingProfileVisitor::new(mapping),
                    ),
                    Err(e) => {
                        error!("{}", e);
                        panic!("Unable to parse profile mapping configuration (yaml file)!");
                    }
                }
            }
            "SwitchStatusProfile" => {
                match serde_yaml::from_str::<SwitchStatusProfileMapping>(&profile_config_string) {
                    Ok(mapping) => VisitorType::SwitchStatus(
                        "switchmodule".into(),
                        SwitchStatusProfile::default(),
                        SwitchStatusProfileVisitor::new(mapping),
                    ),
                    Err(e) => {
                        error!("{}", e);
                        panic!("Unable to parse profile mapping configuration (yaml file)!");
                    }
                }
            }
            _ => {
                panic!("Only Switch Module profiles (Reading, Status, and Control) are supported.");
            }
        }
    }
}

pub fn parse_bit_string(bit_string: &str) -> Option<i32> {
    let mut s = bit_string.to_string();

    s = s.replace("[", "").replace("]", "");

    match isize::from_str_radix(&s, 2) {
        Ok(n) => Some(n as i32),
        Err(e) => None,
    }
}
