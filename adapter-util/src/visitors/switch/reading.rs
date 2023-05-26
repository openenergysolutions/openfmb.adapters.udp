// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use log::error;
use openfmb::messages::commonmodule::*;
use openfmb::messages::switchmodule::*;
use std::collections::HashMap;

pub struct SwitchReadingProfileVisitor {
    pub config: SwitchReadingProfileMapping,
    pub index_map: HashMap<String, usize>,
    pub is_server: bool,
    pub string_setters: HashMap<String, Box<dyn Setter<SwitchReadingProfile, String>>>,
    pub real_setters: HashMap<String, Box<dyn Setter<SwitchReadingProfile, f64>>>,
    pub bool_setters: HashMap<String, Box<dyn Setter<SwitchReadingProfile, bool>>>,
    pub quality_setters: HashMap<String, Box<dyn Setter<SwitchReadingProfile, Quality>>>,
    pub timestamp_setters: HashMap<String, Box<dyn Setter<SwitchReadingProfile, Timestamp>>>,
    pub commands: Vec<Box<dyn Getter<SwitchReadingProfile>>>,
}

impl Visitor for SwitchReadingProfileVisitor {
    type MessageType = SwitchReadingProfile;
    fn visit(&mut self, profile: &mut SwitchReadingProfile) {
        let mapping = self.config.mapping.as_ref().unwrap().clone();
        if let Some(node) = &mapping.reading_message_info {
            self.visit_commonmodule_readingmessageinfo(profile, node);
        }
        if let Some(node) = &mapping.protected_switch {
            self.visit_switchmodule_protectedswitch(profile, node);
        }
        if let Some(node) = &mapping.switch_reading {
            for (i, item) in node.iter().enumerate() {
                if profile.switch_reading_mut().len() == i {
                    let mut vector = profile.switch_reading_mut();
                    vector.push(SwitchReading::default());
                }
                self.index_map.insert("switch_reading".to_string(), i);
                self.visit_switchmodule_switchreading(profile, item);
                self.index_map.remove("switch_reading");
            }
        }
    }
}

impl SwitchReadingProfileVisitor {
    pub fn new(mapping: SwitchReadingProfileMapping) -> SwitchReadingProfileVisitor {
        SwitchReadingProfileVisitor {
            config: mapping,
            index_map: HashMap::new(),
            is_server: false,
            string_setters: HashMap::new(),
            real_setters: HashMap::new(),
            bool_setters: HashMap::new(),
            quality_setters: HashMap::new(),
            timestamp_setters: HashMap::new(),
            commands: Vec::new(),
        }
    }

    pub fn get_string_setters(&mut self) -> Vec<String> {
        self.string_setters
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_real_setters(&mut self) -> Vec<String> {
        self.real_setters
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_bool_setters(&mut self) -> Vec<String> {
        self.bool_setters
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_quality_setters(&mut self) -> Vec<String> {
        self.quality_setters
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_timestamp_setters(&mut self) -> Vec<String> {
        self.timestamp_setters
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_cb_ref(&mut self) -> String {
        match &self.config.cb_ref {
            Some(s) => s.clone(),
            None => "".to_string(),
        }
    }

    pub fn execute_commands(&mut self, profile: &mut SwitchReadingProfile) -> Vec<Command> {
        let mut commands = vec![];
        for cmd in &self.commands {
            if let Some(result) = cmd.execute(profile) {
                commands.push(result);
            }
        }
        self.commands.clear();
        commands
    }

    pub fn device_mrid(&mut self) -> Option<String> {
        if let Some(mrid) = &self
            .config
            .mapping
            .as_ref()?
            .protected_switch
            .as_ref()?
            .conducting_equipment
            .as_ref()?
            .m_rid
            .value
        {
            return Some(mrid.clone());
        }
        None
    }

    fn visit_commonmodule_readingmessageinfo(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.message_info {
            self.visit_commonmodule_messageinfo_2(profile, node);
        }
    }

    fn visit_commonmodule_messageinfo_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &MessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_6(profile, node);
        }
        if let Some(node) = &parent_node.message_time_stamp {
            if &node.field_type == "message" {
                profile
                    .reading_message_info_mut()
                    .message_info_mut()
                    .message_time_stamp = Some(get_current_timestamp());
            }
        }
    }

    fn visit_commonmodule_identifiedobject_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .reading_message_info_mut()
                    .message_info_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .reading_message_info_mut()
                    .message_info_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .reading_message_info_mut()
                    .message_info_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_google_protobuf_stringvalue(
        &mut self,
        parent_node: &StringValueMapping,
    ) -> Option<String> {
        if &parent_node.value.field_type == "constant"
            || &parent_node.value.field_type == "constant_uuid"
            || &parent_node.value.field_type == "primary_uuid"
            || &parent_node.value.field_type == "mapped"
        {
            return Some(parent_node.value.value.as_ref().unwrap().clone());
        } else if &parent_node.value.field_type == "generated_uuid" {
            return Some(new_uuid());
        }
        None
    }

    fn visit_switchmodule_protectedswitch(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ProtectedSwitchMapping,
    ) {
        if let Some(node) = &parent_node.conducting_equipment {
            self.visit_commonmodule_conductingequipment_2(profile, node);
        }
    }

    fn visit_commonmodule_conductingequipment_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ConductingEquipmentMapping,
    ) {
        if let Some(node) = &parent_node.named_object {
            self.visit_commonmodule_namedobject_2(profile, node);
        }
        // Handle String Primitive for mRID
        if &parent_node.m_rid.field_type == "constant"
            || &parent_node.m_rid.field_type == "constant_uuid"
            || &parent_node.m_rid.field_type == "primary_uuid"
            || &parent_node.m_rid.field_type == "mapped"
        {
            profile
                .protected_switch_mut()
                .conducting_equipment_mut()
                .m_rid = parent_node.m_rid.value.as_ref().unwrap().clone();
        }
    }

    fn visit_commonmodule_namedobject_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &NamedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .protected_switch_mut()
                    .conducting_equipment_mut()
                    .named_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .protected_switch_mut()
                    .conducting_equipment_mut()
                    .named_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_switchmodule_switchreading(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &SwitchReadingMapping,
    ) {
        if let Some(node) = &parent_node.conducting_equipment_terminal_reading {
            self.visit_commonmodule_conductingequipmentterminalreading(profile, node);
        }
        if let Some(node) = &parent_node.diff_reading_mmxu {
            self.visit_commonmodule_readingmmxu(profile, node);
        }
        if let Some(node) = &parent_node.phase_mmtn {
            self.visit_commonmodule_phasemmtn(profile, node);
        }
        if let Some(node) = &parent_node.reading_mmtr {
            self.visit_commonmodule_readingmmtr(profile, node);
        }
        if let Some(node) = &parent_node.reading_mmxu {
            self.visit_commonmodule_readingmmxu_1(profile, node);
        }
    }

    fn visit_commonmodule_conductingequipmentterminalreading(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ConductingEquipmentTerminalReadingMapping,
    ) {
        if let Some(node) = &parent_node.terminal {
            self.visit_commonmodule_terminal(profile, node);
        }
    }

    fn visit_commonmodule_terminal(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &TerminalMapping,
    ) {
        if let Some(node) = &parent_node.a_cdc_terminal {
            self.visit_commonmodule_acdcterminal(profile, node);
        }
        if let Some(node) = &parent_node.phases {
            self.visit_commonmodule_optional_phasecodekind(profile, node);
        }
    }

    fn visit_commonmodule_acdcterminal(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ACDCTerminalMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_7(profile, node);
        }
        if let Some(node) = &parent_node.connected {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: bool) {
                        p.switch_reading_mut()[self.switch_reading]
                            .conducting_equipment_terminal_reading_mut()
                            .terminal_mut()
                            .a_cdc_terminal_mut()
                            .connected = Some(value);
                    }
                }
                self.bool_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .conducting_equipment_terminal_reading
                            .as_ref()?
                            .terminal
                            .as_ref()?
                            .a_cdc_terminal
                            .as_ref()?
                            .connected
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: Some(*val),
                                real_value: None,
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::BoolValue(*val, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        if let Some(node) = &parent_node.sequence_number {
            // Handle Int32Value wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .conducting_equipment_terminal_reading_mut()
                            .terminal_mut()
                            .a_cdc_terminal_mut()
                            .sequence_number = Some(value as i32);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .conducting_equipment_terminal_reading
                            .as_ref()?
                            .terminal
                            .as_ref()?
                            .a_cdc_terminal
                            .as_ref()?
                            .sequence_number
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
    }

    fn visit_commonmodule_identifiedobject_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .conducting_equipment_terminal_reading_mut()
                    .terminal_mut()
                    .a_cdc_terminal_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .conducting_equipment_terminal_reading_mut()
                    .terminal_mut()
                    .a_cdc_terminal_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .conducting_equipment_terminal_reading_mut()
                    .terminal_mut()
                    .a_cdc_terminal_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_optional_phasecodekind(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &Optional_PhaseCodeKindMapping,
    ) {
        // Handle Enum for value (non-control)
        if &parent_node.value.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBPhaseCodeKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PhaseCodeKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .conducting_equipment_terminal_reading_mut()
                            .terminal_mut()
                            .phases_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBPhaseCodeKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PhaseCodeKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .conducting_equipment_terminal_reading_mut()
                            .terminal_mut()
                            .phases_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.value.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let value = &p.switch_reading[self.switch_reading]
                        .conducting_equipment_terminal_reading
                        .as_ref()?
                        .terminal
                        .as_ref()?
                        .phases
                        .as_ref()?
                        .value;
                    if let Ok(e) = OpenFMBPhaseCodeKind::try_from(*value) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBPhaseCodeKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*value) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "PhaseCodeKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *value, "PhaseCodeKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_readingmmxu(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMXUMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_2(profile, node);
        }
        if let Some(node) = &parent_node.a {
            self.visit_commonmodule_wye(profile, node);
        }
        if let Some(node) = &parent_node.clc_mth {
            self.visit_commonmodule_eng_calcmethodkind(profile, node);
        }
        if let Some(node) = &parent_node.hz {
            self.visit_commonmodule_mv(profile, node);
        }
        if let Some(node) = &parent_node.pf {
            self.visit_commonmodule_wye_1(profile, node);
        }
        if let Some(node) = &parent_node.pf_sign {
            self.visit_commonmodule_eng_pfsignkind(profile, node);
        }
        if let Some(node) = &parent_node.ph_v {
            self.visit_commonmodule_wye_2(profile, node);
        }
        if let Some(node) = &parent_node.ppv {
            self.visit_commonmodule_del(profile, node);
        }
        if let Some(node) = &parent_node.va {
            self.visit_commonmodule_wye_3(profile, node);
        }
        if let Some(node) = &parent_node.v_ar {
            self.visit_commonmodule_wye_4(profile, node);
        }
        if let Some(node) = &parent_node.w {
            self.visit_commonmodule_wye_5(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_8(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .diff_reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .diff_reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .diff_reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_wye(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_1(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_2(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_3(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_4(profile, node);
        }
    }

    fn visit_commonmodule_cmv(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .a_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_1(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .a_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_2(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .a_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_3(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .a_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_4(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .a_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_eng_calcmethodkind(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ENG_CalcMethodKindMapping,
    ) {
        // Handle Enum for setVal (non-control)
        if &parent_node.set_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum CalcMethodKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .clc_mth_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum CalcMethodKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .clc_mth_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.set_val.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let set_val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .clc_mth
                        .as_ref()?
                        .set_val;
                    if let Ok(e) = OpenFMBCalcMethodKind::try_from(*set_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*set_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "CalcMethodKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *set_val, "CalcMethodKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_mv(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &MVMapping,
    ) {
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .hz_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.units {
            self.visit_commonmodule_unit(profile, node);
        }
    }

    fn visit_commonmodule_unit(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &UnitMapping,
    ) {
        if let Some(node) = &parent_node.multiplier {
            self.visit_commonmodule_optional_unitmultiplierkind(profile, node);
        }
        // Handle Enum for SIUnit (non-control)
        if &parent_node.si_unit.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitSymbolKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .si_unit = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.si_unit.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitSymbolKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .si_unit = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.si_unit.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.si_unit.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let si_unit = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .units
                        .as_ref()?
                        .si_unit;
                    if let Ok(e) = OpenFMBUnitSymbolKind::try_from(*si_unit) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*si_unit) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "UnitSymbolKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *si_unit, "UnitSymbolKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_optional_unitmultiplierkind(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &Optional_UnitMultiplierKindMapping,
    ) {
        // Handle Enum for value (non-control)
        if &parent_node.value.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitMultiplierKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .multiplier_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitMultiplierKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .multiplier_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.value.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let value = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .units
                        .as_ref()?
                        .multiplier
                        .as_ref()?
                        .value;
                    if let Ok(e) = OpenFMBUnitMultiplierKind::try_from(*value) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*value) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "UnitMultiplierKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *value, "UnitMultiplierKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_5(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_6(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_7(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_8(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_9(profile, node);
        }
    }

    fn visit_commonmodule_cmv_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_5(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .pf_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_6(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .pf_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_7(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .pf_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_8(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .pf_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_9(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .pf_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_eng_pfsignkind(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ENG_PFSignKindMapping,
    ) {
        // Handle Enum for setVal (non-control)
        if &parent_node.set_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PFSignKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_sign_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PFSignKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .pf_sign_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.set_val.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let set_val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .pf_sign
                        .as_ref()?
                        .set_val;
                    if let Ok(e) = OpenFMBPFSignKind::try_from(*set_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*set_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "PFSignKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *set_val, "PFSignKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_10(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_11(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_12(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_13(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_14(profile, node);
        }
    }

    fn visit_commonmodule_cmv_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_10(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ph_v_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_11(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_11(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_11(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ph_v_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_12(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_12(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_12(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_13(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_13(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_13(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_14(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_14(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_14(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_del(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &DELMapping,
    ) {
        if let Some(node) = &parent_node.phs_ab {
            self.visit_commonmodule_cmv_15(profile, node);
        }
        if let Some(node) = &parent_node.phs_bc {
            self.visit_commonmodule_cmv_16(profile, node);
        }
        if let Some(node) = &parent_node.phs_ca {
            self.visit_commonmodule_cmv_17(profile, node);
        }
    }

    fn visit_commonmodule_cmv_15(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_15(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_15(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_ab
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ppv_mut()
                        .phs_ab_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_16(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_16(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_16(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_bc
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ppv_mut()
                        .phs_bc_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_17(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_17(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_17(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_ca
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .ppv_mut()
                        .phs_ca_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_18(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_19(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_20(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_21(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_22(profile, node);
        }
    }

    fn visit_commonmodule_cmv_18(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_18(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_18(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .va_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_19(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_19(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_19(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .va_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_20(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_20(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_20(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .va_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_21(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_21(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_21(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .va_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_22(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_22(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_22(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .va_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_23(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_24(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_25(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_26(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_27(profile, node);
        }
    }

    fn visit_commonmodule_cmv_23(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_23(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_23(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .v_ar_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_24(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_24(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_24(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .v_ar_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_25(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_25(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_25(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_26(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_26(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_26(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_27(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_27(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_27(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_28(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_29(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_30(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_31(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_32(profile, node);
        }
    }

    fn visit_commonmodule_cmv_28(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_28(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_28(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .w_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_29(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_29(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_29(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .w_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_30(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_30(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_30(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .w_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_31(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_31(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_31(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .w_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_32(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_32(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_32(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .diff_reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .diff_reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .diff_reading_mmxu_mut()
                        .w_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .diff_reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_phasemmtn(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &PhaseMMTNMapping,
    ) {
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_readingmmtn(profile, node);
        }
        if let Some(node) = &parent_node.phs_ab {
            self.visit_commonmodule_readingmmtn_1(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_readingmmtn_2(profile, node);
        }
        if let Some(node) = &parent_node.phs_bc {
            self.visit_commonmodule_readingmmtn_3(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_readingmmtn_4(profile, node);
        }
        if let Some(node) = &parent_node.phs_ca {
            self.visit_commonmodule_readingmmtn_5(profile, node);
        }
    }

    fn visit_commonmodule_readingmmtn(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_3(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_1(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_2(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_3(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_4(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_5(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_6(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_7(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_8(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_9(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_a_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_a_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_a_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_a_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_a_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtn_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_4(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_9(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_10(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_11(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_12(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_13(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_14(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_15(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_16(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_17(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_10(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ab_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ab_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ab_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_11(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_12(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_13(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_14(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_15(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_16(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_17(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ab_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ab_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtn_2(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_5(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_18(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_19(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_20(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_21(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_22(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_23(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_24(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_25(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_26(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_11(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_11(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_b_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_b_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_b_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_18(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_19(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_20(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_21(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_22(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_23(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_24(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_25(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_26(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_b_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_b_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtn_3(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_6(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_27(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_28(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_29(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_30(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_31(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_32(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_33(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_34(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_35(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_12(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_12(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_bc_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_bc_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_bc_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_27(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_28(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_29(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_30(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_31(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_32(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_33(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_34(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_35(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_bc_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_bc_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtn_4(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_7(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_36(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_37(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_38(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_39(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_40(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_41(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_42(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_43(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_44(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_13(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_13(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_c_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_c_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_c_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_36(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_37(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_38(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_39(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_40(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_41(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_42(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_43(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_44(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_c_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_c_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtn_5(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTNMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_8(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_45(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_46(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_47(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_48(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_49(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_50(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_51(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_52(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_53(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_14(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_14(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ca_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ca_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .phase_mmtn_mut()
                    .phs_ca_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_45(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_46(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_47(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_48(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_49(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_50(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_51(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_52(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_53(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .phase_mmtn_mut()
                        .phs_ca_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .phase_mmtn
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .phase_mmtn_mut()
                            .phs_ca_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmtr(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMTRMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_9(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_ah {
            self.visit_commonmodule_bcr_54(profile, node);
        }
        if let Some(node) = &parent_node.dmd_v_arh {
            self.visit_commonmodule_bcr_55(profile, node);
        }
        if let Some(node) = &parent_node.dmd_wh {
            self.visit_commonmodule_bcr_56(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_ah {
            self.visit_commonmodule_bcr_57(profile, node);
        }
        if let Some(node) = &parent_node.sup_v_arh {
            self.visit_commonmodule_bcr_58(profile, node);
        }
        if let Some(node) = &parent_node.sup_wh {
            self.visit_commonmodule_bcr_59(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_ah {
            self.visit_commonmodule_bcr_60(profile, node);
        }
        if let Some(node) = &parent_node.tot_v_arh {
            self.visit_commonmodule_bcr_61(profile, node);
        }
        if let Some(node) = &parent_node.tot_wh {
            self.visit_commonmodule_bcr_62(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_15(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_15(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmtr_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmtr_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmtr_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_bcr_54(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .dmd_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .dmd_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_55(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .dmd_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .dmd_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_56(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .dmd_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .dmd_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .dmd_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_57(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .sup_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .sup_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_58(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .sup_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .sup_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_59(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .sup_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .sup_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .sup_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_60(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .tot_v_ah_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .tot_v_ah
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_v_ah_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_v_ah_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_61(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .tot_v_arh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .tot_v_arh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_v_arh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_v_arh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_bcr_62(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &BCRMapping,
    ) {
        // Handle Number Primitive for actVal
        if &parent_node.act_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmtr_mut()
                        .tot_wh_mut()
                        .act_val = value as i64;
                }
            }
            self.real_setters.insert(
                parent_node.act_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.act_val.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmtr
                        .as_ref()?
                        .tot_wh
                        .as_ref()?
                        .act_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_wh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmtr_mut()
                            .tot_wh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_readingmmxu_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ReadingMMXUMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_10(profile, node);
        }
        if let Some(node) = &parent_node.a {
            self.visit_commonmodule_wye_6(profile, node);
        }
        if let Some(node) = &parent_node.clc_mth {
            self.visit_commonmodule_eng_calcmethodkind_1(profile, node);
        }
        if let Some(node) = &parent_node.hz {
            self.visit_commonmodule_mv_1(profile, node);
        }
        if let Some(node) = &parent_node.pf {
            self.visit_commonmodule_wye_7(profile, node);
        }
        if let Some(node) = &parent_node.pf_sign {
            self.visit_commonmodule_eng_pfsignkind_1(profile, node);
        }
        if let Some(node) = &parent_node.ph_v {
            self.visit_commonmodule_wye_8(profile, node);
        }
        if let Some(node) = &parent_node.ppv {
            self.visit_commonmodule_del_1(profile, node);
        }
        if let Some(node) = &parent_node.va {
            self.visit_commonmodule_wye_9(profile, node);
        }
        if let Some(node) = &parent_node.v_ar {
            self.visit_commonmodule_wye_10(profile, node);
        }
        if let Some(node) = &parent_node.w {
            self.visit_commonmodule_wye_11(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_16(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_16(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile.switch_reading_mut()[*self.index_map.get("switch_reading").unwrap()]
                    .reading_mmxu_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_wye_6(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_33(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_34(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_35(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_36(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_37(profile, node);
        }
    }

    fn visit_commonmodule_cmv_33(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_33(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_33(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .a_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_34(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_34(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_34(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .a_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_35(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_35(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_35(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .a_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_36(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_36(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_36(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .a_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_37(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_37(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_37(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .a_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .a
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .a_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .a
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_eng_calcmethodkind_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ENG_CalcMethodKindMapping,
    ) {
        // Handle Enum for setVal (non-control)
        if &parent_node.set_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum CalcMethodKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .clc_mth_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum CalcMethodKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .clc_mth_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.set_val.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let set_val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .clc_mth
                        .as_ref()?
                        .set_val;
                    if let Ok(e) = OpenFMBCalcMethodKind::try_from(*set_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBCalcMethodKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*set_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "CalcMethodKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *set_val, "CalcMethodKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_mv_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &MVMapping,
    ) {
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .hz_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.units {
            self.visit_commonmodule_unit_1(profile, node);
        }
    }

    fn visit_commonmodule_unit_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &UnitMapping,
    ) {
        if let Some(node) = &parent_node.multiplier {
            self.visit_commonmodule_optional_unitmultiplierkind_1(profile, node);
        }
        // Handle Enum for SIUnit (non-control)
        if &parent_node.si_unit.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitSymbolKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .si_unit = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.si_unit.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitSymbolKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .si_unit = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.si_unit.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.si_unit.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.si_unit.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let si_unit = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .units
                        .as_ref()?
                        .si_unit;
                    if let Ok(e) = OpenFMBUnitSymbolKind::try_from(*si_unit) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBUnitSymbolKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*si_unit) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "UnitSymbolKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *si_unit, "UnitSymbolKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_optional_unitmultiplierkind_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &Optional_UnitMultiplierKindMapping,
    ) {
        // Handle Enum for value (non-control)
        if &parent_node.value.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitMultiplierKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .multiplier_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum UnitMultiplierKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .hz_mut()
                            .units_mut()
                            .multiplier_mut()
                            .value = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.value.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.value.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let value = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .hz
                        .as_ref()?
                        .units
                        .as_ref()?
                        .multiplier
                        .as_ref()?
                        .value;
                    if let Ok(e) = OpenFMBUnitMultiplierKind::try_from(*value) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBUnitMultiplierKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*value) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "UnitMultiplierKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *value, "UnitMultiplierKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_7(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_38(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_39(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_40(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_41(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_42(profile, node);
        }
    }

    fn visit_commonmodule_cmv_38(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_38(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_38(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .pf_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_39(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_39(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_39(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .pf_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_40(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_40(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_40(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .pf_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_41(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_41(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_41(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .pf_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_42(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_42(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_42(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .pf
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .pf_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_eng_pfsignkind_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &ENG_PFSignKindMapping,
    ) {
        // Handle Enum for setVal (non-control)
        if &parent_node.set_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PFSignKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_sign_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                    switch_reading: usize,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchReadingProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum PFSignKind", value);
                            return;
                        }
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .pf_sign_mut()
                            .set_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.set_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
                switch_reading: usize,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            for item in parent_node.set_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.set_val.name.as_ref().unwrap().clone(),
                    bool_value: None,
                    real_value: item.value,
                    string_value: None,
                    scale: None,
                    priority: 0,
                };
                getter
                    .mapping
                    .insert(item.name.as_ref().unwrap().clone(), cmd);
            }
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let set_val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .pf_sign
                        .as_ref()?
                        .set_val;
                    if let Ok(e) = OpenFMBPFSignKind::try_from(*set_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBPFSignKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*set_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "PFSignKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *set_val, "PFSignKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_8(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_43(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_44(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_45(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_46(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_47(profile, node);
        }
    }

    fn visit_commonmodule_cmv_43(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_43(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_43(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ph_v_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_44(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_44(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_44(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ph_v_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_45(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_45(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_45(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_46(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_46(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_46(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_47(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_47(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_47(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ph_v_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ph_v
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ph_v_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ph_v
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_del_1(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &DELMapping,
    ) {
        if let Some(node) = &parent_node.phs_ab {
            self.visit_commonmodule_cmv_48(profile, node);
        }
        if let Some(node) = &parent_node.phs_bc {
            self.visit_commonmodule_cmv_49(profile, node);
        }
        if let Some(node) = &parent_node.phs_ca {
            self.visit_commonmodule_cmv_50(profile, node);
        }
    }

    fn visit_commonmodule_cmv_48(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_48(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_48(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ab_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_ab
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ppv_mut()
                        .phs_ab_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_ab
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_49(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_49(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_49(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_bc_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_bc
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ppv_mut()
                        .phs_bc_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_bc
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_50(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_50(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_50(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .ppv_mut()
                            .phs_ca_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .ppv
                            .as_ref()?
                            .phs_ca
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .ppv_mut()
                        .phs_ca_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .ppv
                        .as_ref()?
                        .phs_ca
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_9(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_51(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_52(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_53(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_54(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_55(profile, node);
        }
    }

    fn visit_commonmodule_cmv_51(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_51(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_51(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .va_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_52(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_52(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_52(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .va_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_53(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_53(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_53(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .va_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_54(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_54(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_54(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .va_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_55(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_55(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_55(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .va_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .va
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .va_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .va
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_10(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_56(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_57(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_58(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_59(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_60(profile, node);
        }
    }

    fn visit_commonmodule_cmv_56(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_56(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_56(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .v_ar_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_57(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_57(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_57(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .v_ar_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_58(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_58(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_58(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_59(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_59(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_59(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_60(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_60(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_60(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .v_ar_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .v_ar
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .v_ar_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .v_ar
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_wye_11(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &WYEMapping,
    ) {
        if let Some(node) = &parent_node.net {
            self.visit_commonmodule_cmv_61(profile, node);
        }
        if let Some(node) = &parent_node.neut {
            self.visit_commonmodule_cmv_62(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_cmv_63(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_cmv_64(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_cmv_65(profile, node);
        }
    }

    fn visit_commonmodule_cmv_61(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_61(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_61(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .net_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .net
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .w_mut()
                        .net_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .net
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_62(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_62(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_62(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .neut_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .neut
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .w_mut()
                        .neut_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .neut
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_63(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_63(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_63(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_a_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_a
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .w_mut()
                        .phs_a_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_64(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_64(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_64(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_b_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_b
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .w_mut()
                        .phs_b_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_cmv_65(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &CMVMapping,
    ) {
        if let Some(node) = &parent_node.c_val {
            self.visit_commonmodule_vector_65(profile, node);
        }
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Quality) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: Timestamp) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_vector_65(
        &mut self,
        profile: &mut SwitchReadingProfile,
        parent_node: &VectorMapping,
    ) {
        if let Some(node) = &parent_node.ang {
            // Handle DoubleValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {
                    switch_reading: usize,
                }
                let setter = MySetter {
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Setter<SwitchReadingProfile, f64> for MySetter {
                    fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                        p.switch_reading_mut()[self.switch_reading]
                            .reading_mmxu_mut()
                            .w_mut()
                            .phs_c_mut()
                            .c_val_mut()
                            .ang = Some(value as f64);
                    }
                }
                self.real_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                    switch_reading: usize,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                    switch_reading: *self.index_map.get("switch_reading").unwrap(),
                };
                impl Getter<SwitchReadingProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_reading[self.switch_reading]
                            .reading_mmxu
                            .as_ref()?
                            .w
                            .as_ref()?
                            .phs_c
                            .as_ref()?
                            .c_val
                            .as_ref()?
                            .ang
                        {
                            let cmd = ControlMappingOutput {
                                name: self.name.clone(),
                                bool_value: None,
                                real_value: Some((*val) as f64),
                                string_value: None,
                                scale: None,
                                priority: 0,
                            };
                            Some(Command::RealValue((*val) as f64, vec![cmd], None));
                        }
                        None
                    }
                }
                self.commands.push(Box::new(getter));
            }
        }
        // Handle Number Primitive for mag
        if &parent_node.mag.field_type == "mapped" {
            // Start Setter
            struct MySetter {
                switch_reading: usize,
            }
            let setter = MySetter {
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Setter<SwitchReadingProfile, f64> for MySetter {
                fn execute(&self, p: &mut SwitchReadingProfile, value: f64) {
                    p.switch_reading_mut()[self.switch_reading]
                        .reading_mmxu_mut()
                        .w_mut()
                        .phs_c_mut()
                        .c_val_mut()
                        .mag = value as f64;
                }
            }
            self.real_setters.insert(
                parent_node.mag.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
                switch_reading: usize,
            }
            let getter = MyGetter {
                name: parent_node.mag.name.as_ref().unwrap().clone(),
                switch_reading: *self.index_map.get("switch_reading").unwrap(),
            };
            impl Getter<SwitchReadingProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchReadingProfile) -> Option<Command> {
                    let val = &p.switch_reading[self.switch_reading]
                        .reading_mmxu
                        .as_ref()?
                        .w
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .c_val
                        .as_ref()?
                        .mag;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: None,
                        real_value: Some((*val) as f64),
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::RealValue((*val) as f64, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }
}
