#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlMappingOutput {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "string-value")]
    pub string_value: Option<String>,
    #[serde(rename = "bool-value")]
    pub bool_value: Option<bool>,
    #[serde(rename = "real-value")]
    pub real_value: Option<f64>,
    #[serde(rename = "scale")]
    pub scale: Option<f32>,
    #[serde(skip)]
    pub priority: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumFieldType {
    #[serde(rename = "enum-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "mapping")]
    pub mapping: Option<Vec<EnumMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumMapping {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringFieldType {
    #[serde(rename = "string-field-type")]
    pub field_type: String,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoolFieldType {
    #[serde(rename = "bool-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatFieldType {
    #[serde(rename = "float-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleFieldType {
    #[serde(rename = "double-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int32FieldType {
    #[serde(rename = "int32-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int64FieldType {
    #[serde(rename = "int64-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UInt32FieldType {
    #[serde(rename = "int32-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UInt64FieldType {
    #[serde(rename = "int64-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QualityFieldType {
    #[serde(rename = "quality-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimestampFieldType {
    #[serde(rename = "timestamp-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_FaultDirectionKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_PhaseFaultDirectionKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACDMapping {
    #[serde(rename = "dirGeneral")]
    pub dir_general: EnumFieldType,
    #[serde(rename = "dirNeut")]
    pub dir_neut: Option<Optional_PhaseFaultDirectionKindMapping>,
    #[serde(rename = "dirPhsA")]
    pub dir_phs_a: Option<Optional_PhaseFaultDirectionKindMapping>,
    #[serde(rename = "dirPhsB")]
    pub dir_phs_b: Option<Optional_PhaseFaultDirectionKindMapping>,
    #[serde(rename = "dirPhsC")]
    pub dir_phs_c: Option<Optional_PhaseFaultDirectionKindMapping>,
    #[serde(rename = "general")]
    pub general: BoolFieldType,
    #[serde(rename = "neut")]
    pub neut: Option<BoolValueMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<BoolValueMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<BoolValueMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<BoolValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoolValueMapping {
    #[serde(rename = "value")]
    pub value: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifiedObjectMapping {
    #[serde(rename = "description")]
    pub description: Option<StringValueMapping>,
    #[serde(rename = "mRID")]
    pub m_rid: Option<StringValueMapping>,
    #[serde(rename = "name")]
    pub name: Option<StringValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringValueMapping {
    #[serde(rename = "value")]
    pub value: StringFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACDCTerminalMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "connected")]
    pub connected: Option<BoolValueMapping>,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: Option<Int32ValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int32ValueMapping {
    #[serde(rename = "value")]
    pub value: Int32FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_UnitSymbolKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_UnitMultiplierKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivePowerMapping {
    #[serde(rename = "multiplier")]
    pub multiplier: Option<Optional_UnitMultiplierKindMapping>,
    #[serde(rename = "unit")]
    pub unit: Option<Optional_UnitSymbolKindMapping>,
    #[serde(rename = "value")]
    pub value: Option<FloatValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatValueMapping {
    #[serde(rename = "value")]
    pub value: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_PhaseCodeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnitMapping {
    #[serde(rename = "multiplier")]
    pub multiplier: Option<Optional_UnitMultiplierKindMapping>,
    #[serde(rename = "SIUnit")]
    pub si_unit: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_ValidityKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailQualMapping {
    #[serde(rename = "badReference")]
    pub bad_reference: BoolFieldType,
    #[serde(rename = "failure")]
    pub failure: BoolFieldType,
    #[serde(rename = "inaccurate")]
    pub inaccurate: BoolFieldType,
    #[serde(rename = "inconsistent")]
    pub inconsistent: BoolFieldType,
    #[serde(rename = "oldData")]
    pub old_data: BoolFieldType,
    #[serde(rename = "oscillatory")]
    pub oscillatory: BoolFieldType,
    #[serde(rename = "outOfRange")]
    pub out_of_range: BoolFieldType,
    #[serde(rename = "overflow")]
    pub overflow: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_SourceKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QualityMapping {
    #[serde(rename = "quality-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_TimeAccuracyKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeQualityMapping {
    #[serde(rename = "clockFailure")]
    pub clock_failure: BoolFieldType,
    #[serde(rename = "clockNotSynchronized")]
    pub clock_not_synchronized: BoolFieldType,
    #[serde(rename = "leapSecondsKnown")]
    pub leap_seconds_known: BoolFieldType,
    #[serde(rename = "timeAccuracy")]
    pub time_accuracy: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimestampMapping {
    #[serde(rename = "timestamp-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MVMapping {
    #[serde(rename = "mag")]
    pub mag: DoubleFieldType,
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
    #[serde(rename = "units")]
    pub units: Option<UnitMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicalNodeMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalogEventAndStatusGGIOMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "AnIn")]
    pub an_in: Option<MVMapping>,
    #[serde(rename = "Phase")]
    pub phase: Option<Optional_PhaseCodeKindMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedObjectMapping {
    #[serde(rename = "description")]
    pub description: Option<StringValueMapping>,
    #[serde(rename = "name")]
    pub name: Option<StringValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSystemMapping {
    #[serde(rename = "namedObject")]
    pub named_object: Option<NamedObjectMapping>,
    #[serde(rename = "mRID")]
    pub m_rid: StringFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ASGMapping {
    #[serde(rename = "setMag")]
    pub set_mag: DoubleFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BCRMapping {
    #[serde(rename = "actVal")]
    pub act_val: Int64FieldType,
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusSPSMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: BoolFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanEventAndStatusGGIOMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "Ind")]
    pub ind: Option<StatusSPSMapping>,
    #[serde(rename = "Phase")]
    pub phase: Option<Optional_PhaseCodeKindMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageInfoMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "messageTimeStamp")]
    pub message_time_stamp: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityOverrideMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckConditionsMapping {
    #[serde(rename = "interlockCheck")]
    pub interlock_check: Option<BoolValueMapping>,
    #[serde(rename = "synchroCheck")]
    pub synchro_check: Option<BoolValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearingTimeMapping {
    #[serde(rename = "seconds")]
    pub seconds: Int64FieldType,
    #[serde(rename = "nanoseconds")]
    pub nanoseconds: Int32FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorMapping {
    #[serde(rename = "ang")]
    pub ang: Option<DoubleValueMapping>,
    #[serde(rename = "mag")]
    pub mag: DoubleFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleValueMapping {
    #[serde(rename = "value")]
    pub value: DoubleFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CMVMapping {
    #[serde(rename = "cVal")]
    pub c_val: Option<VectorMapping>,
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConductingEquipmentMapping {
    #[serde(rename = "namedObject")]
    pub named_object: Option<NamedObjectMapping>,
    #[serde(rename = "mRID")]
    pub m_rid: StringFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerminalMapping {
    #[serde(rename = "aCDCTerminal")]
    pub a_cdc_terminal: Option<ACDCTerminalMapping>,
    #[serde(rename = "phases")]
    pub phases: Option<Optional_PhaseCodeKindMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConductingEquipmentTerminalReadingMapping {
    #[serde(rename = "terminal")]
    pub terminal: Option<TerminalMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlAPCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: DoubleFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlDPCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlTimestampMapping {
    #[serde(rename = "control-timestamp-field-type")]
    pub field_type: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_ScheduleParameterKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENG_ScheduleParameterMapping {
    #[serde(rename = "scheduleParameterType")]
    pub schedule_parameter_type: String,
    #[serde(rename = "outputs")]
    pub outputs: Option<Vec<ControlMappingOutput>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchedulePointMapping {
    #[serde(rename = "scheduleParameter")]
    pub schedule_parameter: Option<Vec<ENG_ScheduleParameterMapping>>,
    #[serde(rename = "startTime")]
    pub start_time: Option<ControlTimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCSGMapping {
    #[serde(rename = "schpts")]
    pub sch_pts: Option<Vec<SchedulePointMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlScheduleFSCHMapping {
    #[serde(rename = "ValACSG")]
    pub val_acsg: Option<ScheduleCSGMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicalNodeForControlMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlFSCCMapping {
    #[serde(rename = "logicalNodeForControl")]
    pub logical_node_for_control: Option<LogicalNodeForControlMapping>,
    #[serde(rename = "controlScheduleFSCH")]
    pub control_schedule_fsch: Option<ControlScheduleFSCHMapping>,
    #[serde(rename = "islandControlScheduleFSCH")]
    pub island_control_schedule_fsch: Option<ControlScheduleFSCHMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlINCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: Int32FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlINGMapping {
    #[serde(rename = "setVal")]
    pub set_val: Int32FieldType,
    #[serde(rename = "units")]
    pub units: Option<UnitMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlISCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: Int32FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlSPCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "modBlk")]
    pub mod_blk: Option<BoolValueMapping>,
    #[serde(rename = "reset")]
    pub reset: Option<BoolValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CumulativeTimeMapping {
    #[serde(rename = "seconds")]
    pub seconds: Int64FieldType,
    #[serde(rename = "nanoseconds")]
    pub nanoseconds: Int32FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeIntervalMapping {
    #[serde(rename = "end")]
    pub end: Option<Int64ValueMapping>,
    #[serde(rename = "start")]
    pub start: Option<Int64ValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int64ValueMapping {
    #[serde(rename = "value")]
    pub value: Int64FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DELMapping {
    #[serde(rename = "phsAB")]
    pub phs_ab: Option<CMVMapping>,
    #[serde(rename = "phsBC")]
    pub phs_bc: Option<CMVMapping>,
    #[serde(rename = "phsCA")]
    pub phs_ca: Option<CMVMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseDPCMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<ControlDPCMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<ControlDPCMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<ControlDPCMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<ControlDPCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscreteControlXCBRMapping {
    #[serde(rename = "logicalNodeForControl")]
    pub logical_node_for_control: Option<LogicalNodeForControlMapping>,
    #[serde(rename = "Pos")]
    pub pos: Option<PhaseDPCMapping>,
    #[serde(rename = "ProtectionMode")]
    pub protection_mode: Option<ControlINCMapping>,
    #[serde(rename = "RecloseEnabled")]
    pub reclose_enabled: Option<ControlSPCMapping>,
    #[serde(rename = "ResetProtectionPickup")]
    pub reset_protection_pickup: Option<ControlSPCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnergyConsumerMapping {
    #[serde(rename = "conductingEquipment")]
    pub conducting_equipment: Option<ConductingEquipmentMapping>,
    #[serde(rename = "operatingLimit")]
    pub operating_limit: Option<StringValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_CalcMethodKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENG_CalcMethodKindMapping {
    #[serde(rename = "setVal")]
    pub set_val: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_GridConnectModeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENG_GridConnectModeKindMapping {
    #[serde(rename = "setVal")]
    pub set_val: EnumFieldType,
    #[serde(rename = "setValExtension")]
    pub set_val_extension: Option<StringValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_PFSignKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENG_PFSignKindMapping {
    #[serde(rename = "setVal")]
    pub set_val: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_BehaviourModeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_BehaviourModeKindMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_DERGeneratorStateKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_DERGeneratorStateKindMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_DynamicTestKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_DynamicTestKindMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_GridConnectModeKindMapping {
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
    #[serde(rename = "stValExtension")]
    pub st_val_extension: StringFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_HealthKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_HealthKindMapping {
    #[serde(rename = "d")]
    pub d: Option<StringValueMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_SwitchingCapabilityKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ENS_SwitchingCapabilityKindMapping {
    #[serde(rename = "blkEna")]
    pub blk_ena: Option<BoolValueMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDCTEMapping {
    #[serde(rename = "rndDlTmms")]
    pub rnd_dl_tmms: FloatFieldType,
    #[serde(rename = "rtnDlTmms")]
    pub rtn_dl_tmms: FloatFieldType,
    #[serde(rename = "rtnRmpTmms")]
    pub rtn_rmp_tmms: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterServiceAPCMapping {
    #[serde(rename = "enterServiceParameter")]
    pub enter_service_parameter: Option<OperationDCTEMapping>,
    #[serde(rename = "hzHiLim")]
    pub hz_hi_lim: FloatFieldType,
    #[serde(rename = "hzLoLim")]
    pub hz_lo_lim: FloatFieldType,
    #[serde(rename = "rtnSrvAuto")]
    pub rtn_srv_auto: BoolFieldType,
    #[serde(rename = "vHiLim")]
    pub v_hi_lim: FloatFieldType,
    #[serde(rename = "vLoLim")]
    pub v_lo_lim: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ESSMapping {
    #[serde(rename = "conductingEquipment")]
    pub conducting_equipment: Option<ConductingEquipmentMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "modBlk")]
    pub mod_blk: Option<BoolValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastValueSourceMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastIEDMapping {
    #[serde(rename = "forecastValueSource")]
    pub forecast_value_source: Option<ForecastValueSourceMapping>,
    #[serde(rename = "sourceApplicationID")]
    pub source_application_id: StringFieldType,
    #[serde(rename = "sourceDateTime")]
    pub source_date_time: Int64FieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDHFWMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "OplTmmsMax")]
    pub opl_tmms_max: Option<ClearingTimeMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDLFWMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "OplTmmsMax")]
    pub opl_tmms_max: Option<ClearingTimeMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HzWPointMapping {
    #[serde(rename = "deadbandHzVal")]
    pub deadband_hz_val: FloatFieldType,
    #[serde(rename = "slopeVal")]
    pub slope_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HzWAPCMapping {
    #[serde(rename = "overHzWPt")]
    pub over_hz_w_pt: Option<HzWPointMapping>,
    #[serde(rename = "overHzWParameter")]
    pub over_hz_w_parameter: Option<OperationDHFWMapping>,
    #[serde(rename = "underHzWPt")]
    pub under_hz_w_pt: Option<HzWPointMapping>,
    #[serde(rename = "underHzWParameter")]
    pub under_hz_w_parameter: Option<OperationDLFWMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusINSMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: Int32FieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerEventAndStatusGGIOMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "IntIn")]
    pub int_in: Option<StatusINSMapping>,
    #[serde(rename = "Phase")]
    pub phase: Option<Optional_PhaseCodeKindMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDWMXMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDWMNMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitWAPCMapping {
    #[serde(rename = "maxLimParameter")]
    pub max_lim_parameter: Option<OperationDWMXMapping>,
    #[serde(rename = "minLimParameter")]
    pub min_lim_parameter: Option<OperationDWMNMapping>,
    #[serde(rename = "wMaxSptVal")]
    pub w_max_spt_val: FloatFieldType,
    #[serde(rename = "wMinSptVal")]
    pub w_min_spt_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicalNodeForEventAndStatusMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "Beh")]
    pub beh: Option<ENS_BehaviourModeKindMapping>,
    #[serde(rename = "EEHealth")]
    pub ee_health: Option<ENS_HealthKindMapping>,
    #[serde(rename = "HotLineTag")]
    pub hot_line_tag: Option<StatusSPSMapping>,
    #[serde(rename = "RemoteBlk")]
    pub remote_blk: Option<StatusSPSMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeasurementValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterMapping {
    #[serde(rename = "conductingEquipment")]
    pub conducting_equipment: Option<ConductingEquipmentMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameplateValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "model")]
    pub model: Option<StringValueMapping>,
    #[serde(rename = "sernum")]
    pub sernum: Option<StringValueMapping>,
    #[serde(rename = "swRev")]
    pub sw_rev: Option<StringValueMapping>,
    #[serde(rename = "vendor")]
    pub vendor: Option<StringValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDFPFMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "pFExtSet")]
    pub p_f_ext_set: BoolFieldType,
    #[serde(rename = "pFGnTgtMxVal")]
    pub p_f_gn_tgt_mx_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDVARMapping {
    #[serde(rename = "varTgtSpt")]
    pub var_tgt_spt: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDVVRMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "OplTmmsMax")]
    pub opl_tmms_max: Option<ClearingTimeMapping>,
    #[serde(rename = "VRef")]
    pub v_ref: FloatFieldType,
    #[serde(rename = "VRefAdjEna")]
    pub v_ref_adj_ena: BoolFieldType,
    #[serde(rename = "VRefTmms")]
    pub v_ref_tmms: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDVWCMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "OplTmmsMax")]
    pub opl_tmms_max: Option<ClearingTimeMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDWVRMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStorageDFPFMapping {
    #[serde(rename = "operationDFPF")]
    pub operation_dfpf: Option<OperationDFPFMapping>,
    #[serde(rename = "pFLodTgtMxVal")]
    pub p_f_lod_tgt_mx_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptimizationMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PFSPCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: BoolFieldType,
    #[serde(rename = "pFParameter")]
    pub p_f_parameter: Option<OperationDFPFMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PFStorageSPCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: BoolFieldType,
    #[serde(rename = "pFStorageParameter")]
    pub p_f_storage_parameter: Option<OperationStorageDFPFMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseAPCMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<ControlAPCMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<ControlAPCMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<ControlAPCMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<ControlAPCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_DbPosKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusDPSMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: EnumFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseDPSMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<StatusDPSMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<StatusDPSMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<StatusDPSMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<StatusDPSMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseINSMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<StatusINSMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<StatusINSMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<StatusINSMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<StatusINSMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseISCMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<ControlISCMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<ControlISCMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<ControlISCMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<ControlISCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMMTNMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "DmdVAh")]
    pub dmd_v_ah: Option<BCRMapping>,
    #[serde(rename = "DmdVArh")]
    pub dmd_v_arh: Option<BCRMapping>,
    #[serde(rename = "DmdWh")]
    pub dmd_wh: Option<BCRMapping>,
    #[serde(rename = "SupVAh")]
    pub sup_v_ah: Option<BCRMapping>,
    #[serde(rename = "SupVArh")]
    pub sup_v_arh: Option<BCRMapping>,
    #[serde(rename = "SupWh")]
    pub sup_wh: Option<BCRMapping>,
    #[serde(rename = "TotVAh")]
    pub tot_v_ah: Option<BCRMapping>,
    #[serde(rename = "TotVArh")]
    pub tot_v_arh: Option<BCRMapping>,
    #[serde(rename = "TotWh")]
    pub tot_wh: Option<BCRMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseMMTNMapping {
    #[serde(rename = "phsA")]
    pub phs_a: Option<ReadingMMTNMapping>,
    #[serde(rename = "phsAB")]
    pub phs_ab: Option<ReadingMMTNMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<ReadingMMTNMapping>,
    #[serde(rename = "phsBC")]
    pub phs_bc: Option<ReadingMMTNMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<ReadingMMTNMapping>,
    #[serde(rename = "phsCA")]
    pub phs_ca: Option<ReadingMMTNMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_RecloseActionKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseRecloseActionMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<Optional_RecloseActionKindMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<Optional_RecloseActionKindMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<Optional_RecloseActionKindMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<Optional_RecloseActionKindMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseSPCMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<ControlSPCMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<ControlSPCMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<ControlSPCMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<ControlSPCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseSPSMapping {
    #[serde(rename = "phs3")]
    pub phs3: Option<StatusSPSMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<StatusSPSMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<StatusSPSMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<StatusSPSMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PMGMapping {
    #[serde(rename = "net")]
    pub net: Option<MVMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<MVMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<MVMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<MVMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RampRateMapping {
    #[serde(rename = "negativeReactivePowerKVArPerMin")]
    pub negative_reactive_power_kv_ar_per_min: Option<FloatValueMapping>,
    #[serde(rename = "negativeRealPowerKWPerMin")]
    pub negative_real_power_kw_per_min: Option<FloatValueMapping>,
    #[serde(rename = "positiveReactivePowerKVArPerMin")]
    pub positive_reactive_power_kv_ar_per_min: Option<FloatValueMapping>,
    #[serde(rename = "positiveRealPowerKWPerMin")]
    pub positive_real_power_kw_per_min: Option<FloatValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMMTRMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "DmdVAh")]
    pub dmd_v_ah: Option<BCRMapping>,
    #[serde(rename = "DmdVArh")]
    pub dmd_v_arh: Option<BCRMapping>,
    #[serde(rename = "DmdWh")]
    pub dmd_wh: Option<BCRMapping>,
    #[serde(rename = "SupVAh")]
    pub sup_v_ah: Option<BCRMapping>,
    #[serde(rename = "SupVArh")]
    pub sup_v_arh: Option<BCRMapping>,
    #[serde(rename = "SupWh")]
    pub sup_wh: Option<BCRMapping>,
    #[serde(rename = "TotVAh")]
    pub tot_v_ah: Option<BCRMapping>,
    #[serde(rename = "TotVArh")]
    pub tot_v_arh: Option<BCRMapping>,
    #[serde(rename = "TotWh")]
    pub tot_wh: Option<BCRMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WYEMapping {
    #[serde(rename = "net")]
    pub net: Option<CMVMapping>,
    #[serde(rename = "neut")]
    pub neut: Option<CMVMapping>,
    #[serde(rename = "phsA")]
    pub phs_a: Option<CMVMapping>,
    #[serde(rename = "phsB")]
    pub phs_b: Option<CMVMapping>,
    #[serde(rename = "phsC")]
    pub phs_c: Option<CMVMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingMMXUMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "A")]
    pub a: Option<WYEMapping>,
    #[serde(rename = "ClcMth")]
    pub clc_mth: Option<ENG_CalcMethodKindMapping>,
    #[serde(rename = "Hz")]
    pub hz: Option<MVMapping>,
    #[serde(rename = "PF")]
    pub pf: Option<WYEMapping>,
    #[serde(rename = "PFSign")]
    pub pf_sign: Option<ENG_PFSignKindMapping>,
    #[serde(rename = "PhV")]
    pub ph_v: Option<WYEMapping>,
    #[serde(rename = "PPV")]
    pub ppv: Option<DELMapping>,
    #[serde(rename = "VA")]
    pub va: Option<WYEMapping>,
    #[serde(rename = "VAr")]
    pub v_ar: Option<WYEMapping>,
    #[serde(rename = "W")]
    pub w: Option<WYEMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceCapabilityConfigurationMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "AMax")]
    pub a_max: Option<ASGMapping>,
    #[serde(rename = "VAMax")]
    pub va_max: Option<ASGMapping>,
    #[serde(rename = "VarMaxAbs")]
    pub var_max_abs: Option<ASGMapping>,
    #[serde(rename = "VarMaxInj")]
    pub var_max_inj: Option<ASGMapping>,
    #[serde(rename = "VMax")]
    pub v_max: Option<ASGMapping>,
    #[serde(rename = "VMin")]
    pub v_min: Option<ASGMapping>,
    #[serde(rename = "VNom")]
    pub v_nom: Option<ASGMapping>,
    #[serde(rename = "WMax")]
    pub w_max: Option<ASGMapping>,
    #[serde(rename = "WOvrExt")]
    pub w_ovr_ext: Option<ASGMapping>,
    #[serde(rename = "WOvrExtPF")]
    pub w_ovr_ext_pf: Option<ASGMapping>,
    #[serde(rename = "WUndExt")]
    pub w_und_ext: Option<ASGMapping>,
    #[serde(rename = "WUndExtPF")]
    pub w_und_ext_pf: Option<ASGMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_NorOpCatKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_AbnOpCatKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceCapabilityRatingsMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "AbnOpCatRtg")]
    pub abn_op_cat_rtg: EnumFieldType,
    #[serde(rename = "AMaxRtg")]
    pub a_max_rtg: Option<ASGMapping>,
    #[serde(rename = "FreqNomRtg")]
    pub freq_nom_rtg: Option<ASGMapping>,
    #[serde(rename = "NorOpCatRtg")]
    pub nor_op_cat_rtg: EnumFieldType,
    #[serde(rename = "ReactSusceptRtg")]
    pub react_suscept_rtg: Option<ASGMapping>,
    #[serde(rename = "VAMaxRtg")]
    pub va_max_rtg: Option<ASGMapping>,
    #[serde(rename = "VarMaxAbsRtg")]
    pub var_max_abs_rtg: Option<ASGMapping>,
    #[serde(rename = "VarMaxInjRtg")]
    pub var_max_inj_rtg: Option<ASGMapping>,
    #[serde(rename = "VMaxRtg")]
    pub v_max_rtg: Option<ASGMapping>,
    #[serde(rename = "VMinRtg")]
    pub v_min_rtg: Option<ASGMapping>,
    #[serde(rename = "VNomRtg")]
    pub v_nom_rtg: Option<ASGMapping>,
    #[serde(rename = "WMaxRtg")]
    pub w_max_rtg: Option<ASGMapping>,
    #[serde(rename = "WOvrExtRtg")]
    pub w_ovr_ext_rtg: Option<ASGMapping>,
    #[serde(rename = "WOvrExtRtgPF")]
    pub w_ovr_ext_rtg_pf: Option<ASGMapping>,
    #[serde(rename = "WUndExtRtg")]
    pub w_und_ext_rtg: Option<ASGMapping>,
    #[serde(rename = "WUndExtRtgPF")]
    pub w_und_ext_rtg_pf: Option<ASGMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusAndEventXCBRMapping {
    #[serde(rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: Option<LogicalNodeForEventAndStatusMapping>,
    #[serde(rename = "DynamicTest")]
    pub dynamic_test: Option<ENS_DynamicTestKindMapping>,
    #[serde(rename = "Pos")]
    pub pos: Option<PhaseDPSMapping>,
    #[serde(rename = "ProtectionPickup")]
    pub protection_pickup: Option<ACDMapping>,
    #[serde(rename = "ProtectionMode")]
    pub protection_mode: Option<StatusINSMapping>,
    #[serde(rename = "RecloseEnabled")]
    pub reclose_enabled: Option<PhaseSPSMapping>,
    #[serde(rename = "ReclosingAction")]
    pub reclosing_action: Option<PhaseRecloseActionMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusINCMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: Int32FieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusISCMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: Int32FieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMessageInfoMapping {
    #[serde(rename = "messageInfo")]
    pub message_info: Option<MessageInfoMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusValueMapping {
    #[serde(rename = "identifiedObject")]
    pub identified_object: Option<IdentifiedObjectMapping>,
    #[serde(rename = "modBlk")]
    pub mod_blk: Option<BoolValueMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VSSMapping {
    #[serde(rename = "q")]
    pub q: Option<QualityMapping>,
    #[serde(rename = "stVal")]
    pub st_val: StringFieldType,
    #[serde(rename = "t")]
    pub t: Option<TimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringEventAndStatusGGIOMapping {
    #[serde(rename = "logicalNode")]
    pub logical_node: Option<LogicalNodeMapping>,
    #[serde(rename = "Phase")]
    pub phase: Option<Optional_PhaseCodeKindMapping>,
    #[serde(rename = "StrIn")]
    pub str_in: Option<VSSMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchPointMapping {
    #[serde(rename = "Pos")]
    pub pos: Option<ControlDPCMapping>,
    #[serde(rename = "startTime")]
    pub start_time: Option<ControlTimestampMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchCSGMapping {
    #[serde(rename = "crvpts")]
    pub crv_pts: Option<Vec<SwitchPointMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TmHzPointMapping {
    #[serde(rename = "hzVal")]
    pub hz_val: FloatFieldType,
    #[serde(rename = "tmVal")]
    pub tm_val: Option<ClearingTimeMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TmHzCSGMapping {
    #[serde(rename = "overcrvpts")]
    pub over_crv_pts: Option<Vec<TmHzPointMapping>>,
    #[serde(rename = "undercrvpts")]
    pub under_crv_pts: Option<Vec<TmHzPointMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TmVoltPointMapping {
    #[serde(rename = "tmVal")]
    pub tm_val: Option<ClearingTimeMapping>,
    #[serde(rename = "voltVal")]
    pub volt_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TmVoltCSGMapping {
    #[serde(rename = "overcrvpts")]
    pub over_crv_pts: Option<Vec<TmVoltPointMapping>>,
    #[serde(rename = "undercrvpts")]
    pub under_crv_pts: Option<Vec<TmVoltPointMapping>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VarSPCMapping {
    #[serde(rename = "modEna")]
    pub mod_ena: BoolFieldType,
    #[serde(rename = "varParameter")]
    pub var_parameter: Option<OperationDVARMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoltVarPointMapping {
    #[serde(rename = "varVal")]
    pub var_val: FloatFieldType,
    #[serde(rename = "voltVal")]
    pub volt_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoltVarCSGMapping {
    #[serde(rename = "crvpts")]
    pub crv_pts: Option<Vec<VoltVarPointMapping>>,
    #[serde(rename = "vVarParameter")]
    pub v_var_parameter: Option<OperationDVVRMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoltWPointMapping {
    #[serde(rename = "voltVal")]
    pub volt_val: FloatFieldType,
    #[serde(rename = "wVal")]
    pub w_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoltWCSGMapping {
    #[serde(rename = "crvpts")]
    pub crv_pts: Option<Vec<VoltWPointMapping>>,
    #[serde(rename = "voltWParameter")]
    pub volt_w_parameter: Option<OperationDVWCMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VSCMapping {
    #[serde(rename = "ctlVal")]
    pub ctl_val: StringFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WVarPointMapping {
    #[serde(rename = "varVal")]
    pub var_val: FloatFieldType,
    #[serde(rename = "wVal")]
    pub w_val: FloatFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WVarCSGMapping {
    #[serde(rename = "crvpts")]
    pub crv_pts: Option<Vec<WVarPointMapping>>,
    #[serde(rename = "wVarParameter")]
    pub w_var_parameter: Option<OperationDWVRMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_AlrmKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_ControlModeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_DirectionModeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_GridConnectionStateKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_OperatingStateKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_ReactivePowerControlKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_RealPowerControlKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_StateKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Optional_VoltLimitModeKindMapping {
    #[serde(rename = "value")]
    pub value: EnumFieldType,
}
