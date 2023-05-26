#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchReadingProfileMapping {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "control-block")]
    pub cb_ref: Option<String>,
    #[serde(rename = "mapping")]
    pub mapping: Option<SwitchReadingProfileMappingMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchReadingProfileMappingMapping {
    #[serde(rename = "readingMessageInfo")]
    pub reading_message_info: Option<ReadingMessageInfoMapping>,
    #[serde(rename = "protectedSwitch")]
    pub protected_switch: Option<ProtectedSwitchMapping>,
    #[serde(rename = "switchreading")]
    pub switch_reading: Option<Vec<SwitchReadingMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchReadingMapping {
    #[serde(rename = "conductingEquipmentTerminalReading")]
    pub conducting_equipment_terminal_reading: Option<ConductingEquipmentTerminalReadingMapping>,
    #[serde(rename = "diffReadingMMXU")]
    pub diff_reading_mmxu: Option<ReadingMMXUMapping>,
    #[serde(rename = "phaseMMTN")]
    pub phase_mmtn: Option<PhaseMMTNMapping>,
    #[serde(rename = "readingMMTR")]
    pub reading_mmtr: Option<ReadingMMTRMapping>,
    #[serde(rename = "readingMMXU")]
    pub reading_mmxu: Option<ReadingMMXUMapping>,
}
