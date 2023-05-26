// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use log::warn;
use openfmb::messages::switchmodule::*;

impl ConfigReadVisitor<SwitchDiscreteControlProfile> for SwitchDiscreteControlProfileVisitor {
    fn update_boolean(&mut self, key: &str, p: &mut SwitchDiscreteControlProfile, v: bool) {
        match self.bool_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_f64(&mut self, key: &str, p: &mut SwitchDiscreteControlProfile, v: f64) {
        match self.real_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_timestamp(&mut self, key: &str, p: &mut SwitchDiscreteControlProfile, v: Timestamp) {
        match self.timestamp_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_quality(&mut self, key: &str, p: &mut SwitchDiscreteControlProfile, v: Quality) {
        match self.quality_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_string(&mut self, key: &str, p: &mut SwitchDiscreteControlProfile, v: String) {
        match self.string_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
}

impl ConfigReadVisitor<SwitchReadingProfile> for SwitchReadingProfileVisitor {
    fn update_boolean(&mut self, key: &str, p: &mut SwitchReadingProfile, v: bool) {
        match self.bool_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_f64(&mut self, key: &str, p: &mut SwitchReadingProfile, v: f64) {
        match self.real_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_timestamp(&mut self, key: &str, p: &mut SwitchReadingProfile, v: Timestamp) {
        match self.timestamp_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_quality(&mut self, key: &str, p: &mut SwitchReadingProfile, v: Quality) {
        match self.quality_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_string(&mut self, key: &str, p: &mut SwitchReadingProfile, v: String) {
        match self.string_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
}

impl ConfigReadVisitor<SwitchStatusProfile> for SwitchStatusProfileVisitor {
    fn update_boolean(&mut self, key: &str, p: &mut SwitchStatusProfile, v: bool) {
        match self.bool_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_f64(&mut self, key: &str, p: &mut SwitchStatusProfile, v: f64) {
        match self.real_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_timestamp(&mut self, key: &str, p: &mut SwitchStatusProfile, v: Timestamp) {
        match self.timestamp_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_quality(&mut self, key: &str, p: &mut SwitchStatusProfile, v: Quality) {
        match self.quality_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={:?}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
    fn update_string(&mut self, key: &str, p: &mut SwitchStatusProfile, v: String) {
        match self.string_setters.get(key) {
            Some(action) => {
                log::debug!("Action executed for: {}={}.", key, &v);
                action.execute(p, v);
            }
            _ => {
                //warn!("No action found for key {}.", key);
            }
        }
    }
}
