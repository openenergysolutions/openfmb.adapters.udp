// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use log::error;
use openfmb::messages::commonmodule::*;
use openfmb::messages::switchmodule::*;
use std::collections::HashMap;

pub struct SwitchDiscreteControlProfileVisitor {
    pub config: SwitchDiscreteControlProfileMapping,
    pub index_map: HashMap<String, usize>,
    pub is_server: bool,
    pub commands: Vec<Box<dyn Getter<SwitchDiscreteControlProfile>>>,
    pub command_priority: CommandPriorityMap,
    pub string_setters: HashMap<String, Box<dyn Setter<SwitchDiscreteControlProfile, String>>>,
    pub real_setters: HashMap<String, Box<dyn Setter<SwitchDiscreteControlProfile, f64>>>,
    pub bool_setters: HashMap<String, Box<dyn Setter<SwitchDiscreteControlProfile, bool>>>,
    pub quality_setters: HashMap<String, Box<dyn Setter<SwitchDiscreteControlProfile, Quality>>>,
    pub timestamp_setters:
        HashMap<String, Box<dyn Setter<SwitchDiscreteControlProfile, Timestamp>>>,
}

impl Visitor for SwitchDiscreteControlProfileVisitor {
    type MessageType = SwitchDiscreteControlProfile;
    fn visit(&mut self, profile: &mut SwitchDiscreteControlProfile) {
        let mapping = self.config.mapping.as_ref().unwrap().clone();
        if let Some(node) = &mapping.control_message_info {
            self.visit_commonmodule_controlmessageinfo(profile, node);
        }
        if let Some(node) = &mapping.protected_switch {
            self.visit_switchmodule_protectedswitch(profile, node);
        }
        if let Some(node) = &mapping.switch_discrete_control {
            self.visit_switchmodule_switchdiscretecontrol(profile, node);
        }
    }
}

impl SwitchDiscreteControlProfileVisitor {
    pub fn new(
        mapping: SwitchDiscreteControlProfileMapping,
    ) -> SwitchDiscreteControlProfileVisitor {
        SwitchDiscreteControlProfileVisitor {
            command_priority: CommandPriorityMap::new(mapping.command_order.clone()),
            config: mapping,
            index_map: HashMap::new(),
            is_server: false,
            commands: Vec::new(),
            string_setters: HashMap::new(),
            real_setters: HashMap::new(),
            bool_setters: HashMap::new(),
            quality_setters: HashMap::new(),
            timestamp_setters: HashMap::new(),
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

    pub fn execute_commands(&mut self, profile: &mut SwitchDiscreteControlProfile) -> Vec<Command> {
        let mut commands = vec![];
        for cmd in &self.commands {
            if let Some(result) = cmd.execute(profile) {
                commands.push(result);
            }
        }
        self.commands.clear();
        commands
    }

    pub fn get_tolerance_ms(&mut self) -> Option<u32> {
        self.config.tolerance_ms
    }

    pub fn get_command_orders(&mut self) -> Option<Vec<String>> {
        self.config.command_order.clone()
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

    fn visit_commonmodule_controlmessageinfo(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlMessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.message_info {
            self.visit_commonmodule_messageinfo(profile, node);
        }
    }

    fn visit_commonmodule_messageinfo(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &MessageInfoMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject(profile, node);
        }
        if let Some(node) = &parent_node.message_time_stamp {
            if self.is_server {
                profile
                    .control_message_info_mut()
                    .message_info_mut()
                    .message_time_stamp = Some(get_current_timestamp());
            }
        }
    }

    fn visit_commonmodule_identifiedobject(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.m_rid {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.name {
            // StringValue wrapper type is not supported for Control profiles
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
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ProtectedSwitchMapping,
    ) {
        if let Some(node) = &parent_node.conducting_equipment {
            self.visit_commonmodule_conductingequipment(profile, node);
        }
    }

    fn visit_commonmodule_conductingequipment(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ConductingEquipmentMapping,
    ) {
        if let Some(node) = &parent_node.named_object {
            self.visit_commonmodule_namedobject(profile, node);
        }
        // Handle String Primitive for mRID
        if self.is_server {
            if &parent_node.m_rid.field_type == "constant" {
                profile
                    .protected_switch_mut()
                    .conducting_equipment_mut()
                    .m_rid = parent_node.m_rid.value.as_ref().unwrap().clone();
            }
        } else {
            // String type is not supported for Control profiles.  Set string-field-type=constant to always initialize this field
        }
    }

    fn visit_commonmodule_namedobject(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &NamedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.name {
            // StringValue wrapper type is not supported for Control profiles
        }
    }

    fn visit_switchmodule_switchdiscretecontrol(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &SwitchDiscreteControlMapping,
    ) {
        if let Some(node) = &parent_node.control_value {
            self.visit_commonmodule_controlvalue(profile, node);
        }
        if let Some(node) = &parent_node.check {
            self.visit_commonmodule_checkconditions(profile, node);
        }
        if let Some(node) = &parent_node.switch_discrete_control_xswi {
            self.visit_switchmodule_switchdiscretecontrolxswi(profile, node);
        }
    }

    fn visit_commonmodule_controlvalue(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlValueMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_1(profile, node);
        }
        if let Some(node) = &parent_node.mod_blk {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                        p.switch_discrete_control_mut().control_value_mut().mod_blk = Some(value);
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
                impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                        if let Some(val) = &p
                            .switch_discrete_control
                            .as_ref()?
                            .control_value
                            .as_ref()?
                            .mod_blk
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
        if let Some(node) = &parent_node.reset {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                        p.switch_discrete_control_mut().control_value_mut().reset = Some(value);
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
                impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                        if let Some(val) = &p
                            .switch_discrete_control
                            .as_ref()?
                            .control_value
                            .as_ref()?
                            .reset
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

    fn visit_commonmodule_identifiedobject_1(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.m_rid {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.name {
            // StringValue wrapper type is not supported for Control profiles
        }
    }

    fn visit_commonmodule_checkconditions(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &CheckConditionsMapping,
    ) {
        if let Some(node) = &parent_node.interlock_check {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                        p.switch_discrete_control_mut().check_mut().interlock_check = Some(value);
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
                impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                        if let Some(val) = &p
                            .switch_discrete_control
                            .as_ref()?
                            .check
                            .as_ref()?
                            .interlock_check
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
        if let Some(node) = &parent_node.synchro_check {
            // Handle BoolValue wrapper type
            if &node.value.field_type == "mapped" {
                struct MySetter {}
                let setter = MySetter {};
                impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                        p.switch_discrete_control_mut().check_mut().synchro_check = Some(value);
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
                impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                    fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                        if let Some(val) = &p
                            .switch_discrete_control
                            .as_ref()?
                            .check
                            .as_ref()?
                            .synchro_check
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

    fn visit_switchmodule_switchdiscretecontrolxswi(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &SwitchDiscreteControlXSWIMapping,
    ) {
        if let Some(node) = &parent_node.logical_node_for_control {
            self.visit_commonmodule_logicalnodeforcontrol(profile, node);
        }
        if let Some(node) = &parent_node.pos {
            self.visit_commonmodule_phasedpc(profile, node);
        }
        if let Some(node) = &parent_node.reset_protection_pickup {
            self.visit_commonmodule_controlspc(profile, node);
        }
    }

    fn visit_commonmodule_logicalnodeforcontrol(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &LogicalNodeForControlMapping,
    ) {
        if let Some(node) = &parent_node.logical_node {
            self.visit_commonmodule_logicalnode(profile, node);
        }
    }

    fn visit_commonmodule_logicalnode(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &LogicalNodeMapping,
    ) {
        if let Some(node) = &parent_node.identified_object {
            self.visit_commonmodule_identifiedobject_2(profile, node);
        }
    }

    fn visit_commonmodule_identifiedobject_2(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &IdentifiedObjectMapping,
    ) {
        if let Some(node) = &parent_node.description {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.m_rid {
            // StringValue wrapper type is not supported for Control profiles
        }
        if let Some(node) = &parent_node.name {
            // StringValue wrapper type is not supported for Control profiles
        }
    }

    fn visit_commonmodule_phasedpc(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &PhaseDPCMapping,
    ) {
        if let Some(node) = &parent_node.phs3 {
            self.visit_commonmodule_controldpc(profile, node);
        }
        if let Some(node) = &parent_node.phs_a {
            self.visit_commonmodule_controldpc_1(profile, node);
        }
        if let Some(node) = &parent_node.phs_b {
            self.visit_commonmodule_controldpc_2(profile, node);
        }
        if let Some(node) = &parent_node.phs_c {
            self.visit_commonmodule_controldpc_3(profile, node);
        }
    }

    fn visit_commonmodule_controldpc(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlDPCMapping,
    ) {
        // Handle Bool Primitive for ctlVal
        if &parent_node.ctl_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                    p.switch_discrete_control_mut()
                        .switch_discrete_control_xswi_mut()
                        .pos_mut()
                        .phs3_mut()
                        .ctl_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.ctl_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.ctl_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                    let val = &p
                        .switch_discrete_control
                        .as_ref()?
                        .switch_discrete_control_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs3
                        .as_ref()?
                        .ctl_val;
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
    }

    fn visit_commonmodule_controldpc_1(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlDPCMapping,
    ) {
        // Handle Bool Primitive for ctlVal
        if &parent_node.ctl_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                    p.switch_discrete_control_mut()
                        .switch_discrete_control_xswi_mut()
                        .pos_mut()
                        .phs_a_mut()
                        .ctl_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.ctl_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.ctl_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                    let val = &p
                        .switch_discrete_control
                        .as_ref()?
                        .switch_discrete_control_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_a
                        .as_ref()?
                        .ctl_val;
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
    }

    fn visit_commonmodule_controldpc_2(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlDPCMapping,
    ) {
        // Handle Bool Primitive for ctlVal
        if &parent_node.ctl_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                    p.switch_discrete_control_mut()
                        .switch_discrete_control_xswi_mut()
                        .pos_mut()
                        .phs_b_mut()
                        .ctl_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.ctl_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.ctl_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                    let val = &p
                        .switch_discrete_control
                        .as_ref()?
                        .switch_discrete_control_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_b
                        .as_ref()?
                        .ctl_val;
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
    }

    fn visit_commonmodule_controldpc_3(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlDPCMapping,
    ) {
        // Handle Bool Primitive for ctlVal
        if &parent_node.ctl_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                    p.switch_discrete_control_mut()
                        .switch_discrete_control_xswi_mut()
                        .pos_mut()
                        .phs_c_mut()
                        .ctl_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.ctl_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.ctl_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                    let val = &p
                        .switch_discrete_control
                        .as_ref()?
                        .switch_discrete_control_xswi
                        .as_ref()?
                        .pos
                        .as_ref()?
                        .phs_c
                        .as_ref()?
                        .ctl_val;
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
    }

    fn visit_commonmodule_controlspc(
        &mut self,
        profile: &mut SwitchDiscreteControlProfile,
        parent_node: &ControlSPCMapping,
    ) {
        // Handle Bool Primitive for ctlVal
        if &parent_node.ctl_val.field_type == "mapped" {
            // Start Setter
            struct MySetter {}
            let setter = MySetter {};
            impl Setter<SwitchDiscreteControlProfile, bool> for MySetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile, value: bool) {
                    p.switch_discrete_control_mut()
                        .switch_discrete_control_xswi_mut()
                        .reset_protection_pickup_mut()
                        .ctl_val = value;
                }
            }
            self.bool_setters.insert(
                parent_node.ctl_val.name.as_ref().unwrap().to_string(),
                Box::new(setter),
            );
            // End Setter
            // Start Getter
            struct MyGetter {
                name: String,
            }
            let getter = MyGetter {
                name: parent_node.ctl_val.name.as_ref().unwrap().clone(),
            };
            impl Getter<SwitchDiscreteControlProfile> for MyGetter {
                fn execute(&self, p: &mut SwitchDiscreteControlProfile) -> Option<Command> {
                    let val = &p
                        .switch_discrete_control
                        .as_ref()?
                        .switch_discrete_control_xswi
                        .as_ref()?
                        .reset_protection_pickup
                        .as_ref()?
                        .ctl_val;
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
    }
}
