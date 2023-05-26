// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use log::error;
use openfmb::messages::commonmodule::*;
use openfmb::messages::switchmodule::*;
use std::collections::HashMap;

pub struct SwitchStatusProfileVisitor {
    pub config: SwitchStatusProfileMapping,
    pub index_map: HashMap<String, usize>,
    pub is_server: bool,
    pub string_setters: HashMap<String, Box<dyn Setter<SwitchStatusProfile, String>>>,
    pub real_setters: HashMap<String, Box<dyn Setter<SwitchStatusProfile, f64>>>,
    pub bool_setters: HashMap<String, Box<dyn Setter<SwitchStatusProfile, bool>>>,
    pub quality_setters: HashMap<String, Box<dyn Setter<SwitchStatusProfile, Quality>>>,
    pub timestamp_setters: HashMap<String, Box<dyn Setter<SwitchStatusProfile, Timestamp>>>,
    pub commands: Vec<Box<dyn Getter<SwitchStatusProfile>>>,
}

impl Visitor for SwitchStatusProfileVisitor {
    type MessageType = SwitchStatusProfile;
    fn visit(&mut self, profile: &mut SwitchStatusProfile) {
        let mapping = self.config.mapping.as_ref().unwrap().clone();
        if let Some(node) = &mapping.status_message_info {
            self.visit_commonmodule_statusmessageinfo(profile, node);
        }
        if let Some(node) = &mapping.protected_switch {
            self.visit_switchmodule_protectedswitch(profile, node);
        }
        if let Some(node) = &mapping.switch_status {
            self.visit_switchmodule_switchstatus(profile, node);
        }
    }
}

impl SwitchStatusProfileVisitor {
    pub fn new(mapping: SwitchStatusProfileMapping) -> SwitchStatusProfileVisitor {
        SwitchStatusProfileVisitor {
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

    pub fn execute_commands(&mut self, profile: &mut SwitchStatusProfile) -> Vec<Command> {
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

    fn visit_commonmodule_statusmessageinfo(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusMessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.message_info {
            self.visit_commonmodule_messageinfo_3(profile, node);
        }
    }

    fn visit_commonmodule_messageinfo_3(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &MessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_17(profile, node);
        }
        if let Some(node) = &parent_node.message_time_stamp {
            if &node.field_type == "message" {
                profile
                    .status_message_info_mut()
                    .message_info_mut()
                    .message_time_stamp = Some(get_current_timestamp());
            }
        }
    }

    fn visit_commonmodule_identifiedobject_17(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .status_message_info_mut()
                    .message_info_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .status_message_info_mut()
                    .message_info_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .status_message_info_mut()
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
        profile: &mut SwitchStatusProfile,
        parent_node: &ProtectedSwitchMapping,
    ) {
        if let Some(node) = &parent_node.conducting_equipment {
            self.visit_commonmodule_conductingequipment_3(profile, node);
        }
    }

    fn visit_commonmodule_conductingequipment_3(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &ConductingEquipmentMapping,
    ) {
        if let Some(node) = &parent_node.named_object {
            self.visit_commonmodule_namedobject_3(profile, node);
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

    fn visit_commonmodule_namedobject_3(
        &mut self,
        profile: &mut SwitchStatusProfile,
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

    fn visit_switchmodule_switchstatus(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &SwitchStatusMapping,
    ) {
        if let Some(node) = &parent_node.status_value {
            self.visit_commonmodule_statusvalue(profile, node);
        }
        if let Some(node) = &parent_node.switch_status_xswi {
            self.visit_switchmodule_switchstatusxswi(profile, node);
        }
    }

    fn visit_commonmodule_statusvalue(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusValueMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_18(profile, node);
        }
        if let Some(node) = &parent_node.mod_blk {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                        p.switch_status_mut().status_value_mut().mod_blk = Some(value);
                    }
                }
                self.bool_setters.insert(
                    node.value.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                struct MyGetter {
                    name: String,
                }
                let getter = MyGetter {
                    name: node.value.name.as_ref().unwrap().to_string(),
                };
                impl Getter<SwitchStatusProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                        if let Some(val) = &p.switch_status.as_ref()?.status_value.as_ref()?.mod_blk
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
    }

    fn visit_commonmodule_identifiedobject_18(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .status_value_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .status_value_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .status_value_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_switchmodule_switchstatusxswi(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &SwitchStatusXSWIMapping,
    ) {
        if let Some(node) = &parent_node.logical_node_for_event_and_status {
            self.visit_commonmodule_logicalnodeforeventandstatus_1(profile, node);
        }
        if let Some(node) = &parent_node.dynamic_test {
            self.visit_commonmodule_ens_dynamictestkind_1(profile, node);
        }
        if let Some(node) = &parent_node.pos {
            self.visit_commonmodule_phasedps_1(profile, node);
        }
        if let Some(node) = &parent_node.protection_pickup {
            self.visit_commonmodule_phasesps(profile, node);
        }
    }

    fn visit_commonmodule_logicalnodeforeventandstatus_1(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &LogicalNodeForEventAndStatusMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode_11(profile, node);
        }
        if let Some(node) = &parent_node.beh {
            self.visit_commonmodule_ens_behaviourmodekind_1(profile, node);
        }
        if let Some(node) = &parent_node.ee_health {
            self.visit_commonmodule_ens_healthkind_1(profile, node);
        }
        if let Some(node) = &parent_node.hot_line_tag {
            self.visit_commonmodule_statussps_2(profile, node);
        }
        if let Some(node) = &parent_node.remote_blk {
            self.visit_commonmodule_statussps_3(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode_11(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_19(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_19(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .switch_status_xswi_mut()
                    .logical_node_for_event_and_status_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .description = Some(s);
            }
        }
        if let Some(node) = &parent_node.m_rid {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .switch_status_xswi_mut()
                    .logical_node_for_event_and_status_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .m_rid = Some(s);
            }
        }
        if let Some(node) = &parent_node.name {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .switch_status_xswi_mut()
                    .logical_node_for_event_and_status_mut()
                    .logical_node_mut()
                    .identified_object_mut()
                    .name = Some(s);
            }
        }
    }

    fn visit_commonmodule_ens_behaviourmodekind_1(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &ENS_BehaviourModeKindMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .beh_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBBehaviourModeKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum BehaviourModeKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .beh_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBBehaviourModeKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum BehaviourModeKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .beh_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .logical_node_for_event_and_status
                        .as_ref()?
                        .beh
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBBehaviourModeKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBBehaviourModeKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "BehaviourModeKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "BehaviourModeKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .beh_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_ens_healthkind_1(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &ENS_HealthKindMapping,
    ) {
        if let Some(node) = &parent_node.d {
            // Handle StringValue wrapper type
            if let Some(s) = self.visit_google_protobuf_stringvalue(node) {
                profile
                    .switch_status_mut()
                    .switch_status_xswi_mut()
                    .logical_node_for_event_and_status_mut()
                    .ee_health_mut()
                    .d = Some(s);
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBHealthKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum HealthKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .ee_health_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBHealthKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum HealthKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .ee_health_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .logical_node_for_event_and_status
                        .as_ref()?
                        .ee_health
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBHealthKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBHealthKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "HealthKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "HealthKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
    }

    fn visit_commonmodule_statussps_2(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .hot_line_tag_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .logical_node_for_event_and_status_mut()
                        .hot_line_tag_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .logical_node_for_event_and_status
                        .as_ref()?
                        .hot_line_tag
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .hot_line_tag_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statussps_3(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .remote_blk_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .logical_node_for_event_and_status_mut()
                        .remote_blk_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .logical_node_for_event_and_status
                        .as_ref()?
                        .remote_blk
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .logical_node_for_event_and_status_mut()
                            .remote_blk_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_ens_dynamictestkind_1(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &ENS_DynamicTestKindMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .dynamic_test_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBDynamicTestKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DynamicTestKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .dynamic_test_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBDynamicTestKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DynamicTestKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .dynamic_test_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .dynamic_test
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBDynamicTestKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBDynamicTestKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "DynamicTestKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "DynamicTestKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .dynamic_test_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_phasedps_1(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &PhaseDPSMapping,
    ) {
        if let Some(node) = &parent_node.phs3 {
            self.visit_commonmodule_statusdps_4(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_statusdps_5(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_statusdps_6(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_statusdps_7(profile, node);
        }
    }

    fn visit_commonmodule_statusdps_4(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusDPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs3_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs3_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs3_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs3
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBDbPosKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "DbPosKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "DbPosKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs3_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statusdps_5(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusDPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_a_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_a_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBDbPosKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "DbPosKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "DbPosKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statusdps_6(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusDPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_b_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_b_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBDbPosKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "DbPosKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "DbPosKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statusdps_7(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusDPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Enum for stVal (non-control)
        if &parent_node.st_val.field_type == "mapped" {
            if self.is_server {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, String>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        item.value.as_ref().unwrap().to_string(),
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        for (k, v) in &self.mapping {
                            if *v == value {
                                if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                    set_val = s as i32;
                                    found = true;
                                }
                                break;
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_c_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
            // End Setter
            } else {
                // Start Setter
                struct MySetter {
                    mapping: HashMap<String, i32>,
                }
                let mut setter = MySetter {
                    mapping: HashMap::new(),
                };
                for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                    setter.mapping.insert(
                        item.name.as_ref().unwrap().clone(),
                        *item.value.as_ref().unwrap() as i32,
                    );
                }
                impl Setter<SwitchStatusProfile, String> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: String) {
                        use std::str::FromStr;
                        let mut set_val: i32 = 0;
                        let mut found = false;
                        if let Some(val_as_int) = parse_bit_string(&value) {
                            for (k, v) in &self.mapping {
                                if *v == val_as_int {
                                    if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                        set_val = s as i32;
                                        found = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !found {
                            error!("Unable to map {} to enum DbPosKind", value);
                            return;
                        }
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_c_mut()
                            .st_val = set_val;
                    }
                }
                self.string_setters.insert(
                    parent_node.st_val.name.as_ref().unwrap().to_string(),
                    Box::new(setter),
                );
                // End Setter
            }
            // Start Getter
            struct MyGetter {
                mapping: HashMap<String, ControlMappingOutput>,
            }
            let mut getter = MyGetter {
                mapping: HashMap::new(),
            };
            for item in parent_node.st_val.mapping.as_ref().unwrap().iter() {
                let cmd = ControlMappingOutput {
                    name: parent_node.st_val.name.as_ref().unwrap().clone(),
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
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    use std::convert::TryFrom;
                    use std::str::FromStr;
                    let st_val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .st_val;
                    if let Ok(e) = OpenFMBDbPosKind::try_from(*st_val) {
                        for (k, v) in &self.mapping {
                            if let Ok(s) = OpenFMBDbPosKind::from_str(k) {
                                if s == e {
                                    return Some(Command::RealValue(
                                        (*st_val) as f64,
                                        vec![v.clone()],
                                        None,
                                    ));
                                }
                            } else {
                                error!(
                                    "Unable to map '{}' in configuration to '{}'.",
                                    &k, "DbPosKind"
                                );
                            }
                        }
                    } else {
                        error!(
                            "Unable to map  value '{}' from message to '{}'.",
                            *st_val, "DbPosKind"
                        );
                    }
                    None
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .pos_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_phasesps(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &PhaseSPSMapping,
    ) {
        if let Some(node) = &parent_node.phs3 {
            self.visit_commonmodule_statussps_4(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_statussps_5(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_statussps_6(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_statussps_7(profile, node);
        }
    }

    fn visit_commonmodule_statussps_4(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs3_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .protection_pickup_mut()
                        .phs3_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .protection_pickup
                        .as_ref()?
                        .phs3
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs3_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statussps_5(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_a_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .protection_pickup_mut()
                        .phs_a_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .protection_pickup
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_a_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statussps_6(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_b_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .protection_pickup_mut()
                        .phs_b_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .protection_pickup
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_b_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }

    fn visit_commonmodule_statussps_7(
        &mut self,
        profile: &mut SwitchStatusProfile,
        parent_node: &StatusSPSMapping,
    ) {
        if let Some(node) = &parent_node.q {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Quality> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Quality) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_c_mut()
                            .q = Some(value);
                    }
                }
                self.quality_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
        // Handle Bool Primitive for stVal
        if &parent_node.st_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchStatusProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchStatusProfile, value: bool) {
                    p.switch_status_mut()
                        .switch_status_xswi_mut()
                        .protection_pickup_mut()
                        .phs_c_mut()
                        .st_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.st_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.st_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchStatusProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchStatusProfile) -> Option<Command> {
                    let val = &p
                        .switch_status
                        .as_ref()?
                        .switch_status_xswi
                        .as_ref()?
                        .protection_pickup
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .st_val;
                    let cmd = ControlMappingOutput {
                        name: self.name.clone(),
                        bool_value: Some(*val),
                        real_value: None,
                        string_value: None,
                        scale: None,
                        priority: 0,
                    };
                    Some(Command::BoolValue(*val, vec![cmd], None))
                }
            }
            self.commands.push(Box::new(getter));
            // End Getter
        }
        if let Some(node) = &parent_node.t {
            if &node.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchStatusProfile, Timestamp> for MySetter {
                    fn execute(&self, p: &mut SwitchStatusProfile, value: Timestamp) {
                        p.switch_status_mut()
                            .switch_status_xswi_mut()
                            .protection_pickup_mut()
                            .phs_c_mut()
                            .t = Some(value);
                    }
                }
                self.timestamp_setters
                    .insert(node.name.as_ref().unwrap().to_string(), Box::new(setter));
            }
        }
    }
}
