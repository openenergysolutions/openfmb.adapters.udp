#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchDiscreteControlProfileMapping {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "control-block")]
    pub cb_ref: Option<String>,
    #[serde(rename = "tolerance-ms")]
    pub tolerance_ms: Option<u32>,
    #[serde(rename = "command-order")]
    pub command_order: Option<Vec<String>>,
    #[serde(rename = "mapping")]
    pub mapping: Option<SwitchDiscreteControlProfileMappingMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchDiscreteControlProfileMappingMapping {
    #[serde(rename = "controlMessageInfo")]
    pub control_message_info: Option<ControlMessageInfoMapping>,
    #[serde(rename = "protectedSwitch")]
    pub protected_switch: Option<ProtectedSwitchMapping>,
    #[serde(rename = "switchDiscreteControl")]
    pub switch_discrete_control: Option<SwitchDiscreteControlMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedSwitchMapping {
    #[serde(rename = "conductingEquipment")]
    pub conducting_equipment: Option<ConductingEquipmentMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchDiscreteControlMapping {
    #[serde(rename = "controlValue")]
    pub control_value: Option<ControlValueMapping>,
    #[serde(rename = "check")]
    pub check: Option<CheckConditionsMapping>,
    #[serde(rename = "switchDiscreteControlXSWI")]
    pub switch_discrete_control_xswi: Option<SwitchDiscreteControlXSWIMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchDiscreteControlXSWIMapping {
    #[serde(rename = "logicalNodeForControl")]
    pub logical_node_for_control: Option<LogicalNodeForControlMapping>,
    #[serde(rename = "Pos")]
    pub pos: Option<PhaseDPCMapping>,
    #[serde(rename = "ResetProtectionPickup")]
    pub reset_protection_pickup: Option<ControlSPCMapping>,
}
