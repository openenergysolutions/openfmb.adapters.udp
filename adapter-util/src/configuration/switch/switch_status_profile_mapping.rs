#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatusProfileMapping {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "control-block")]
    pub cb_ref: Option<String>,
    #[serde(rename = "mapping")]
    pub mapping: Option<SwitchStatusProfileMappingMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatusProfileMappingMapping {
    #[serde(rename = "statusMessageInfo")]
    pub status_message_info: Option<StatusMessageInfoMapping>,
    #[serde(rename = "protectedSwitch")]
    pub protected_switch: Option<ProtectedSwitchMapping>,
    #[serde(rename = "switchStatus")]
    pub switch_status: Option<SwitchStatusMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatusMapping {
    #[serde(rename = "statusValue")]
    pub status_value: Option<StatusValueMapping>,
    #[serde(rename = "switchStatusXSWI")]
    pub switch_status_xswi: Option<SwitchStatusXSWIMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatusXSWIMapping {
    #[serde(rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: Option<LogicalNodeForEventAndStatusMapping>,
    #[serde(rename = "DynamicTest")]
    pub dynamic_test: Option<ENS_DynamicTestKindMapping>,
    #[serde(rename = "Pos")]
    pub pos: Option<PhaseDPSMapping>,
    #[serde(rename = "ProtectionPickup")]
    pub protection_pickup: Option<PhaseSPSMapping>,
}
