#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::common::*;
use serde::{Deserialize, Serialize};

pub mod switch_discrete_control_profile_mapping;
pub use switch_discrete_control_profile_mapping::*;

pub mod switch_reading_profile_mapping;
pub use switch_reading_profile_mapping::*;

pub mod switch_status_profile_mapping;
pub use switch_status_profile_mapping::*;
