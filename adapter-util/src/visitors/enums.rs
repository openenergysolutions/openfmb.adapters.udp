// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryFrom;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBBehaviourModeKind {
    BehaviourModeKind_UNDEFINED = 0,
    BehaviourModeKind_on = 1,
    BehaviourModeKind_blocked = 2,
    BehaviourModeKind_test = 3,
    BehaviourModeKind_test_blocked = 4,
    BehaviourModeKind_off = 5,
}
impl FromStr for OpenFMBBehaviourModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBBehaviourModeKind, Self::Err> {
        match input {
            "BehaviourModeKind_UNDEFINED" => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_UNDEFINED)
            }
            "BehaviourModeKind_on" => Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_on),
            "BehaviourModeKind_blocked" => Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_blocked),
            "BehaviourModeKind_test" => Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_test),
            "BehaviourModeKind_test_blocked" => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_test_blocked)
            }
            "BehaviourModeKind_off" => Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_off),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBBehaviourModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_UNDEFINED as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_UNDEFINED)
            }
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_on as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_on)
            }
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_blocked as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_blocked)
            }
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_test as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_test)
            }
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_test_blocked as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_test_blocked)
            }
            x if x == OpenFMBBehaviourModeKind::BehaviourModeKind_off as i32 => {
                Ok(OpenFMBBehaviourModeKind::BehaviourModeKind_off)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBHealthKind {
    HealthKind_UNDEFINED = 0,
    HealthKind_none = 1,
    HealthKind_OK = 2,
    HealthKind_Warning = 3,
    HealthKind_Alarm = 4,
}
impl FromStr for OpenFMBHealthKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBHealthKind, Self::Err> {
        match input {
            "HealthKind_UNDEFINED" => Ok(OpenFMBHealthKind::HealthKind_UNDEFINED),
            "HealthKind_none" => Ok(OpenFMBHealthKind::HealthKind_none),
            "HealthKind_OK" => Ok(OpenFMBHealthKind::HealthKind_OK),
            "HealthKind_Warning" => Ok(OpenFMBHealthKind::HealthKind_Warning),
            "HealthKind_Alarm" => Ok(OpenFMBHealthKind::HealthKind_Alarm),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBHealthKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBHealthKind::HealthKind_UNDEFINED as i32 => {
                Ok(OpenFMBHealthKind::HealthKind_UNDEFINED)
            }
            x if x == OpenFMBHealthKind::HealthKind_none as i32 => {
                Ok(OpenFMBHealthKind::HealthKind_none)
            }
            x if x == OpenFMBHealthKind::HealthKind_OK as i32 => {
                Ok(OpenFMBHealthKind::HealthKind_OK)
            }
            x if x == OpenFMBHealthKind::HealthKind_Warning as i32 => {
                Ok(OpenFMBHealthKind::HealthKind_Warning)
            }
            x if x == OpenFMBHealthKind::HealthKind_Alarm as i32 => {
                Ok(OpenFMBHealthKind::HealthKind_Alarm)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBDynamicTestKind {
    DynamicTestKind_UNDEFINED = 0,
    DynamicTestKind_none = 1,
    DynamicTestKind_testing = 2,
    DynamicTestKind_operating = 3,
    DynamicTestKind_failed = 4,
}
impl FromStr for OpenFMBDynamicTestKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBDynamicTestKind, Self::Err> {
        match input {
            "DynamicTestKind_UNDEFINED" => Ok(OpenFMBDynamicTestKind::DynamicTestKind_UNDEFINED),
            "DynamicTestKind_none" => Ok(OpenFMBDynamicTestKind::DynamicTestKind_none),
            "DynamicTestKind_testing" => Ok(OpenFMBDynamicTestKind::DynamicTestKind_testing),
            "DynamicTestKind_operating" => Ok(OpenFMBDynamicTestKind::DynamicTestKind_operating),
            "DynamicTestKind_failed" => Ok(OpenFMBDynamicTestKind::DynamicTestKind_failed),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBDynamicTestKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBDynamicTestKind::DynamicTestKind_UNDEFINED as i32 => {
                Ok(OpenFMBDynamicTestKind::DynamicTestKind_UNDEFINED)
            }
            x if x == OpenFMBDynamicTestKind::DynamicTestKind_none as i32 => {
                Ok(OpenFMBDynamicTestKind::DynamicTestKind_none)
            }
            x if x == OpenFMBDynamicTestKind::DynamicTestKind_testing as i32 => {
                Ok(OpenFMBDynamicTestKind::DynamicTestKind_testing)
            }
            x if x == OpenFMBDynamicTestKind::DynamicTestKind_operating as i32 => {
                Ok(OpenFMBDynamicTestKind::DynamicTestKind_operating)
            }
            x if x == OpenFMBDynamicTestKind::DynamicTestKind_failed as i32 => {
                Ok(OpenFMBDynamicTestKind::DynamicTestKind_failed)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBDbPosKind {
    DbPosKind_UNDEFINED = 0,
    DbPosKind_transient = 1,
    DbPosKind_closed = 2,
    DbPosKind_open = 3,
    DbPosKind_invalid = 4,
}
impl FromStr for OpenFMBDbPosKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBDbPosKind, Self::Err> {
        match input {
            "DbPosKind_UNDEFINED" => Ok(OpenFMBDbPosKind::DbPosKind_UNDEFINED),
            "DbPosKind_transient" => Ok(OpenFMBDbPosKind::DbPosKind_transient),
            "DbPosKind_closed" => Ok(OpenFMBDbPosKind::DbPosKind_closed),
            "DbPosKind_open" => Ok(OpenFMBDbPosKind::DbPosKind_open),
            "DbPosKind_invalid" => Ok(OpenFMBDbPosKind::DbPosKind_invalid),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBDbPosKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBDbPosKind::DbPosKind_UNDEFINED as i32 => {
                Ok(OpenFMBDbPosKind::DbPosKind_UNDEFINED)
            }
            x if x == OpenFMBDbPosKind::DbPosKind_transient as i32 => {
                Ok(OpenFMBDbPosKind::DbPosKind_transient)
            }
            x if x == OpenFMBDbPosKind::DbPosKind_closed as i32 => {
                Ok(OpenFMBDbPosKind::DbPosKind_closed)
            }
            x if x == OpenFMBDbPosKind::DbPosKind_open as i32 => {
                Ok(OpenFMBDbPosKind::DbPosKind_open)
            }
            x if x == OpenFMBDbPosKind::DbPosKind_invalid as i32 => {
                Ok(OpenFMBDbPosKind::DbPosKind_invalid)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBPhaseCodeKind {
    PhaseCodeKind_none = 0,
    PhaseCodeKind_other = 1,
    PhaseCodeKind_N = 16,
    PhaseCodeKind_C = 32,
    PhaseCodeKind_CN = 33,
    PhaseCodeKind_AC = 40,
    PhaseCodeKind_ACN = 41,
    PhaseCodeKind_B = 64,
    PhaseCodeKind_BN = 65,
    PhaseCodeKind_BC = 66,
    PhaseCodeKind_BCN = 97,
    PhaseCodeKind_A = 128,
    PhaseCodeKind_AN = 129,
    PhaseCodeKind_AB = 132,
    PhaseCodeKind_ABN = 193,
    PhaseCodeKind_ABC = 224,
    PhaseCodeKind_ABCN = 225,
    PhaseCodeKind_s2 = 256,
    PhaseCodeKind_s2N = 257,
    PhaseCodeKind_s1 = 512,
    PhaseCodeKind_s1N = 513,
    PhaseCodeKind_s12 = 768,
    PhaseCodeKind_s12N = 769,
}
impl FromStr for OpenFMBPhaseCodeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBPhaseCodeKind, Self::Err> {
        match input {
            "PhaseCodeKind_none" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_none),
            "PhaseCodeKind_other" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_other),
            "PhaseCodeKind_N" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_N),
            "PhaseCodeKind_C" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_C),
            "PhaseCodeKind_CN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_CN),
            "PhaseCodeKind_AC" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AC),
            "PhaseCodeKind_ACN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ACN),
            "PhaseCodeKind_B" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_B),
            "PhaseCodeKind_BN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BN),
            "PhaseCodeKind_BC" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BC),
            "PhaseCodeKind_BCN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BCN),
            "PhaseCodeKind_A" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_A),
            "PhaseCodeKind_AN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AN),
            "PhaseCodeKind_AB" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AB),
            "PhaseCodeKind_ABN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABN),
            "PhaseCodeKind_ABC" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABC),
            "PhaseCodeKind_ABCN" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABCN),
            "PhaseCodeKind_s2" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s2),
            "PhaseCodeKind_s2N" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s2N),
            "PhaseCodeKind_s1" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s1),
            "PhaseCodeKind_s1N" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s1N),
            "PhaseCodeKind_s12" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s12),
            "PhaseCodeKind_s12N" => Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s12N),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBPhaseCodeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_none as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_none)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_other as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_other)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_N as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_N)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_C as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_C)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_CN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_CN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_AC as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AC)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_ACN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ACN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_B as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_B)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_BN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_BC as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BC)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_BCN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_BCN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_A as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_A)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_AN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_AB as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_AB)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_ABN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_ABC as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABC)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_ABCN as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_ABCN)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s2 as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s2)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s2N as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s2N)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s1 as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s1)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s1N as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s1N)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s12 as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s12)
            }
            x if x == OpenFMBPhaseCodeKind::PhaseCodeKind_s12N as i32 => {
                Ok(OpenFMBPhaseCodeKind::PhaseCodeKind_s12N)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBCalcMethodKind {
    CalcMethodKind_UNDEFINED = 0,
    CalcMethodKind_P_CLASS = 11,
    CalcMethodKind_M_CLASS = 12,
    CalcMethodKind_DIFF = 13,
}
impl FromStr for OpenFMBCalcMethodKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBCalcMethodKind, Self::Err> {
        match input {
            "CalcMethodKind_UNDEFINED" => Ok(OpenFMBCalcMethodKind::CalcMethodKind_UNDEFINED),
            "CalcMethodKind_P_CLASS" => Ok(OpenFMBCalcMethodKind::CalcMethodKind_P_CLASS),
            "CalcMethodKind_M_CLASS" => Ok(OpenFMBCalcMethodKind::CalcMethodKind_M_CLASS),
            "CalcMethodKind_DIFF" => Ok(OpenFMBCalcMethodKind::CalcMethodKind_DIFF),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBCalcMethodKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBCalcMethodKind::CalcMethodKind_UNDEFINED as i32 => {
                Ok(OpenFMBCalcMethodKind::CalcMethodKind_UNDEFINED)
            }
            x if x == OpenFMBCalcMethodKind::CalcMethodKind_P_CLASS as i32 => {
                Ok(OpenFMBCalcMethodKind::CalcMethodKind_P_CLASS)
            }
            x if x == OpenFMBCalcMethodKind::CalcMethodKind_M_CLASS as i32 => {
                Ok(OpenFMBCalcMethodKind::CalcMethodKind_M_CLASS)
            }
            x if x == OpenFMBCalcMethodKind::CalcMethodKind_DIFF as i32 => {
                Ok(OpenFMBCalcMethodKind::CalcMethodKind_DIFF)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBUnitMultiplierKind {
    UnitMultiplierKind_UNDEFINED = 0,
    UnitMultiplierKind_none = 1,
    UnitMultiplierKind_other = 2,
    UnitMultiplierKind_centi = 3,
    UnitMultiplierKind_deci = 4,
    UnitMultiplierKind_Giga = 5,
    UnitMultiplierKind_kilo = 6,
    UnitMultiplierKind_Mega = 7,
    UnitMultiplierKind_micro = 8,
    UnitMultiplierKind_milli = 9,
    UnitMultiplierKind_nano = 10,
    UnitMultiplierKind_pico = 11,
    UnitMultiplierKind_Tera = 12,
}
impl FromStr for OpenFMBUnitMultiplierKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBUnitMultiplierKind, Self::Err> {
        match input {
            "UnitMultiplierKind_UNDEFINED" => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_UNDEFINED)
            }
            "UnitMultiplierKind_none" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_none),
            "UnitMultiplierKind_other" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_other),
            "UnitMultiplierKind_centi" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_centi),
            "UnitMultiplierKind_deci" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_deci),
            "UnitMultiplierKind_Giga" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Giga),
            "UnitMultiplierKind_kilo" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_kilo),
            "UnitMultiplierKind_Mega" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Mega),
            "UnitMultiplierKind_micro" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_micro),
            "UnitMultiplierKind_milli" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_milli),
            "UnitMultiplierKind_nano" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_nano),
            "UnitMultiplierKind_pico" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_pico),
            "UnitMultiplierKind_Tera" => Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Tera),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBUnitMultiplierKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_UNDEFINED as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_UNDEFINED)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_none as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_none)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_other as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_other)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_centi as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_centi)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_deci as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_deci)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_Giga as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Giga)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_kilo as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_kilo)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_Mega as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Mega)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_micro as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_micro)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_milli as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_milli)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_nano as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_nano)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_pico as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_pico)
            }
            x if x == OpenFMBUnitMultiplierKind::UnitMultiplierKind_Tera as i32 => {
                Ok(OpenFMBUnitMultiplierKind::UnitMultiplierKind_Tera)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBUnitSymbolKind {
    UnitSymbolKind_none = 0,
    UnitSymbolKind_meter = 2,
    UnitSymbolKind_gram = 3,
    UnitSymbolKind_Amp = 5,
    UnitSymbolKind_deg = 9,
    UnitSymbolKind_rad = 10,
    UnitSymbolKind_degC = 23,
    UnitSymbolKind_Farad = 25,
    UnitSymbolKind_sec = 27,
    UnitSymbolKind_Henry = 28,
    UnitSymbolKind_V = 29,
    UnitSymbolKind_ohm = 30,
    UnitSymbolKind_Joule = 31,
    UnitSymbolKind_Newton = 32,
    UnitSymbolKind_Hz = 33,
    UnitSymbolKind_W = 38,
    UnitSymbolKind_Pa = 39,
    UnitSymbolKind_m2 = 41,
    UnitSymbolKind_Siemens = 53,
    UnitSymbolKind_VA = 61,
    UnitSymbolKind_VAr = 63,
    UnitSymbolKind_wPerVA = 65,
    UnitSymbolKind_VAh = 71,
    UnitSymbolKind_Wh = 72,
    UnitSymbolKind_VArh = 73,
    UnitSymbolKind_hzPerS = 75,
    UnitSymbolKind_wPerS = 81,
    UnitSymbolKind_other = 100,
    UnitSymbolKind_Ah = 106,
    UnitSymbolKind_min = 159,
    UnitSymbolKind_hour = 160,
    UnitSymbolKind_m3 = 166,
    UnitSymbolKind_wPerM2 = 179,
    UnitSymbolKind_degF = 279,
    UnitSymbolKind_mph = 500,
}
impl FromStr for OpenFMBUnitSymbolKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBUnitSymbolKind, Self::Err> {
        match input {
            "UnitSymbolKind_none" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_none),
            "UnitSymbolKind_meter" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_meter),
            "UnitSymbolKind_gram" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_gram),
            "UnitSymbolKind_Amp" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Amp),
            "UnitSymbolKind_deg" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_deg),
            "UnitSymbolKind_rad" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_rad),
            "UnitSymbolKind_degC" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_degC),
            "UnitSymbolKind_Farad" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Farad),
            "UnitSymbolKind_sec" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_sec),
            "UnitSymbolKind_Henry" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Henry),
            "UnitSymbolKind_V" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_V),
            "UnitSymbolKind_ohm" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_ohm),
            "UnitSymbolKind_Joule" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Joule),
            "UnitSymbolKind_Newton" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Newton),
            "UnitSymbolKind_Hz" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Hz),
            "UnitSymbolKind_W" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_W),
            "UnitSymbolKind_Pa" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Pa),
            "UnitSymbolKind_m2" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_m2),
            "UnitSymbolKind_Siemens" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Siemens),
            "UnitSymbolKind_VA" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VA),
            "UnitSymbolKind_VAr" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VAr),
            "UnitSymbolKind_wPerVA" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerVA),
            "UnitSymbolKind_VAh" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VAh),
            "UnitSymbolKind_Wh" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Wh),
            "UnitSymbolKind_VArh" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VArh),
            "UnitSymbolKind_hzPerS" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_hzPerS),
            "UnitSymbolKind_wPerS" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerS),
            "UnitSymbolKind_other" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_other),
            "UnitSymbolKind_Ah" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Ah),
            "UnitSymbolKind_min" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_min),
            "UnitSymbolKind_hour" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_hour),
            "UnitSymbolKind_m3" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_m3),
            "UnitSymbolKind_wPerM2" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerM2),
            "UnitSymbolKind_degF" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_degF),
            "UnitSymbolKind_mph" => Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_mph),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBUnitSymbolKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_none as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_none)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_meter as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_meter)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_gram as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_gram)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Amp as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Amp)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_deg as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_deg)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_rad as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_rad)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_degC as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_degC)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Farad as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Farad)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_sec as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_sec)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Henry as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Henry)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_V as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_V)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_ohm as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_ohm)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Joule as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Joule)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Newton as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Newton)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Hz as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Hz)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_W as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_W)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Pa as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Pa)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_m2 as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_m2)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Siemens as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Siemens)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_VA as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VA)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_VAr as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VAr)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_wPerVA as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerVA)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_VAh as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VAh)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Wh as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Wh)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_VArh as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_VArh)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_hzPerS as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_hzPerS)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_wPerS as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerS)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_other as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_other)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_Ah as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_Ah)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_min as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_min)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_hour as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_hour)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_m3 as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_m3)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_wPerM2 as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_wPerM2)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_degF as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_degF)
            }
            x if x == OpenFMBUnitSymbolKind::UnitSymbolKind_mph as i32 => {
                Ok(OpenFMBUnitSymbolKind::UnitSymbolKind_mph)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBPFSignKind {
    PFSignKind_UNDEFINED = 0,
    PFSignKind_IEC = 1,
    PFSignKind_EEI = 2,
}
impl FromStr for OpenFMBPFSignKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBPFSignKind, Self::Err> {
        match input {
            "PFSignKind_UNDEFINED" => Ok(OpenFMBPFSignKind::PFSignKind_UNDEFINED),
            "PFSignKind_IEC" => Ok(OpenFMBPFSignKind::PFSignKind_IEC),
            "PFSignKind_EEI" => Ok(OpenFMBPFSignKind::PFSignKind_EEI),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBPFSignKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBPFSignKind::PFSignKind_UNDEFINED as i32 => {
                Ok(OpenFMBPFSignKind::PFSignKind_UNDEFINED)
            }
            x if x == OpenFMBPFSignKind::PFSignKind_IEC as i32 => {
                Ok(OpenFMBPFSignKind::PFSignKind_IEC)
            }
            x if x == OpenFMBPFSignKind::PFSignKind_EEI as i32 => {
                Ok(OpenFMBPFSignKind::PFSignKind_EEI)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBAbnOpCatKind {
    AbnOpCatKind_UNDEFINED = 0,
    AbnOpCatKind_I = 1,
    AbnOpCatKind_II = 2,
    AbnOpCatKind_III = 3,
}
impl FromStr for OpenFMBAbnOpCatKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBAbnOpCatKind, Self::Err> {
        match input {
            "AbnOpCatKind_UNDEFINED" => Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_UNDEFINED),
            "AbnOpCatKind_I" => Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_I),
            "AbnOpCatKind_II" => Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_II),
            "AbnOpCatKind_III" => Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_III),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBAbnOpCatKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBAbnOpCatKind::AbnOpCatKind_UNDEFINED as i32 => {
                Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_UNDEFINED)
            }
            x if x == OpenFMBAbnOpCatKind::AbnOpCatKind_I as i32 => {
                Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_I)
            }
            x if x == OpenFMBAbnOpCatKind::AbnOpCatKind_II as i32 => {
                Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_II)
            }
            x if x == OpenFMBAbnOpCatKind::AbnOpCatKind_III as i32 => {
                Ok(OpenFMBAbnOpCatKind::AbnOpCatKind_III)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBNorOpCatKind {
    NorOpCatKind_UNDEFINED = 0,
    NorOpCatKind_A = 1,
    NorOpCatKind_B = 2,
}
impl FromStr for OpenFMBNorOpCatKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBNorOpCatKind, Self::Err> {
        match input {
            "NorOpCatKind_UNDEFINED" => Ok(OpenFMBNorOpCatKind::NorOpCatKind_UNDEFINED),
            "NorOpCatKind_A" => Ok(OpenFMBNorOpCatKind::NorOpCatKind_A),
            "NorOpCatKind_B" => Ok(OpenFMBNorOpCatKind::NorOpCatKind_B),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBNorOpCatKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBNorOpCatKind::NorOpCatKind_UNDEFINED as i32 => {
                Ok(OpenFMBNorOpCatKind::NorOpCatKind_UNDEFINED)
            }
            x if x == OpenFMBNorOpCatKind::NorOpCatKind_A as i32 => {
                Ok(OpenFMBNorOpCatKind::NorOpCatKind_A)
            }
            x if x == OpenFMBNorOpCatKind::NorOpCatKind_B as i32 => {
                Ok(OpenFMBNorOpCatKind::NorOpCatKind_B)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBScheduleParameterKind {
    ScheduleParameterKind_UNDEFINED = 0,
    ScheduleParameterKind_none = 1,
    ScheduleParameterKind_other = 2,
    ScheduleParameterKind_A_net_mag = 3,
    ScheduleParameterKind_A_neut_mag = 4,
    ScheduleParameterKind_A_phsA_mag = 5,
    ScheduleParameterKind_A_phsB_mag = 6,
    ScheduleParameterKind_A_phsC_mag = 7,
    ScheduleParameterKind_Hz_mag = 8,
    ScheduleParameterKind_PF_net_mag = 9,
    ScheduleParameterKind_PF_neut_mag = 10,
    ScheduleParameterKind_PF_phsA_mag = 11,
    ScheduleParameterKind_PF_phsB_mag = 12,
    ScheduleParameterKind_PF_phsC_mag = 13,
    ScheduleParameterKind_PhV_net_ang = 14,
    ScheduleParameterKind_PhV_net_mag = 15,
    ScheduleParameterKind_PhV_neut_ang = 16,
    ScheduleParameterKind_PhV_neut_mag = 17,
    ScheduleParameterKind_PhV_phsA_ang = 18,
    ScheduleParameterKind_PhV_phsA_mag = 19,
    ScheduleParameterKind_PhV_phsB_ang = 20,
    ScheduleParameterKind_PhV_phsB_mag = 21,
    ScheduleParameterKind_PhV_phsC_ang = 22,
    ScheduleParameterKind_PhV_phsC_mag = 23,
    ScheduleParameterKind_PPV_phsAB_ang = 24,
    ScheduleParameterKind_PPV_phsAB_mag = 25,
    ScheduleParameterKind_PPV_phsBC_ang = 26,
    ScheduleParameterKind_PPV_phsBC_mag = 27,
    ScheduleParameterKind_PPV_phsCA_ang = 28,
    ScheduleParameterKind_PPV_phsCA_mag = 29,
    ScheduleParameterKind_VA_net_mag = 30,
    ScheduleParameterKind_VA_neut_mag = 31,
    ScheduleParameterKind_VA_phsA_mag = 32,
    ScheduleParameterKind_VA_phsB_mag = 33,
    ScheduleParameterKind_VA_phsC_mag = 34,
    ScheduleParameterKind_VAr_net_mag = 35,
    ScheduleParameterKind_VAr_neut_mag = 36,
    ScheduleParameterKind_VAr_phsA_mag = 37,
    ScheduleParameterKind_VAr_phsB_mag = 38,
    ScheduleParameterKind_VAr_phsC_mag = 39,
    ScheduleParameterKind_W_net_mag = 40,
    ScheduleParameterKind_W_neut_mag = 41,
    ScheduleParameterKind_W_phsA_mag = 42,
    ScheduleParameterKind_W_phsB_mag = 43,
    ScheduleParameterKind_W_phsC_mag = 44,
}
impl FromStr for OpenFMBScheduleParameterKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBScheduleParameterKind, Self::Err> {
        match input {
            "ScheduleParameterKind_UNDEFINED" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_UNDEFINED)
            }
            "ScheduleParameterKind_none" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_none)
            }
            "ScheduleParameterKind_other" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_other)
            }
            "ScheduleParameterKind_A_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_net_mag)
            }
            "ScheduleParameterKind_A_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_neut_mag)
            }
            "ScheduleParameterKind_A_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsA_mag)
            }
            "ScheduleParameterKind_A_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsB_mag)
            }
            "ScheduleParameterKind_A_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsC_mag)
            }
            "ScheduleParameterKind_Hz_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_Hz_mag)
            }
            "ScheduleParameterKind_PF_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_net_mag)
            }
            "ScheduleParameterKind_PF_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_neut_mag)
            }
            "ScheduleParameterKind_PF_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsA_mag)
            }
            "ScheduleParameterKind_PF_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsB_mag)
            }
            "ScheduleParameterKind_PF_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsC_mag)
            }
            "ScheduleParameterKind_PhV_net_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_ang)
            }
            "ScheduleParameterKind_PhV_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_mag)
            }
            "ScheduleParameterKind_PhV_neut_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_ang)
            }
            "ScheduleParameterKind_PhV_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_mag)
            }
            "ScheduleParameterKind_PhV_phsA_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_ang)
            }
            "ScheduleParameterKind_PhV_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_mag)
            }
            "ScheduleParameterKind_PhV_phsB_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_ang)
            }
            "ScheduleParameterKind_PhV_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_mag)
            }
            "ScheduleParameterKind_PhV_phsC_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_ang)
            }
            "ScheduleParameterKind_PhV_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_mag)
            }
            "ScheduleParameterKind_PPV_phsAB_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_ang)
            }
            "ScheduleParameterKind_PPV_phsAB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_mag)
            }
            "ScheduleParameterKind_PPV_phsBC_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_ang)
            }
            "ScheduleParameterKind_PPV_phsBC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_mag)
            }
            "ScheduleParameterKind_PPV_phsCA_ang" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_ang)
            }
            "ScheduleParameterKind_PPV_phsCA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_mag)
            }
            "ScheduleParameterKind_VA_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_net_mag)
            }
            "ScheduleParameterKind_VA_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_neut_mag)
            }
            "ScheduleParameterKind_VA_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsA_mag)
            }
            "ScheduleParameterKind_VA_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsB_mag)
            }
            "ScheduleParameterKind_VA_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsC_mag)
            }
            "ScheduleParameterKind_VAr_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_net_mag)
            }
            "ScheduleParameterKind_VAr_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_neut_mag)
            }
            "ScheduleParameterKind_VAr_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsA_mag)
            }
            "ScheduleParameterKind_VAr_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsB_mag)
            }
            "ScheduleParameterKind_VAr_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsC_mag)
            }
            "ScheduleParameterKind_W_net_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_net_mag)
            }
            "ScheduleParameterKind_W_neut_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_neut_mag)
            }
            "ScheduleParameterKind_W_phsA_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsA_mag)
            }
            "ScheduleParameterKind_W_phsB_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsB_mag)
            }
            "ScheduleParameterKind_W_phsC_mag" => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsC_mag)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBScheduleParameterKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_UNDEFINED as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_UNDEFINED)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_none as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_none)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_other as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_other)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_A_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_A_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_A_phsC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_Hz_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_Hz_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PF_phsC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PhV_phsC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsAB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsBC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_ang as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_ang)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_PPV_phsCA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VA_phsC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_VAr_phsC_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_W_net_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_net_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_W_neut_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_neut_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsA_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsA_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsB_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsB_mag)
            }
            x if x == OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsC_mag as i32 => {
                Ok(OpenFMBScheduleParameterKind::ScheduleParameterKind_W_phsC_mag)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBGridConnectModeKind {
    GridConnectModeKind_UNDEFINED = 0,
    GridConnectModeKind_CSI = 1,
    GridConnectModeKind_VC_VSI = 2,
    GridConnectModeKind_CC_VSI = 3,
    GridConnectModeKind_none = 98,
    GridConnectModeKind_other = 99,
    GridConnectModeKind_VSI_PQ = 2000,
    GridConnectModeKind_VSI_VF = 2001,
    GridConnectModeKind_VSI_ISO = 2002,
}
impl FromStr for OpenFMBGridConnectModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBGridConnectModeKind, Self::Err> {
        match input {
            "GridConnectModeKind_UNDEFINED" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_UNDEFINED)
            }
            "GridConnectModeKind_CSI" => Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_CSI),
            "GridConnectModeKind_VC_VSI" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VC_VSI)
            }
            "GridConnectModeKind_CC_VSI" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_CC_VSI)
            }
            "GridConnectModeKind_none" => Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_none),
            "GridConnectModeKind_other" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_other)
            }
            "GridConnectModeKind_VSI_PQ" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_PQ)
            }
            "GridConnectModeKind_VSI_VF" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_VF)
            }
            "GridConnectModeKind_VSI_ISO" => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_ISO)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBGridConnectModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_UNDEFINED as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_UNDEFINED)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_CSI as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_CSI)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_VC_VSI as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VC_VSI)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_CC_VSI as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_CC_VSI)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_none as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_none)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_other as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_other)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_PQ as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_PQ)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_VF as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_VF)
            }
            x if x == OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_ISO as i32 => {
                Ok(OpenFMBGridConnectModeKind::GridConnectModeKind_VSI_ISO)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBStateKind {
    StateKind_UNDEFINED = 0,
    StateKind_off = 1,
    StateKind_on = 2,
    StateKind_standby = 3,
}
impl FromStr for OpenFMBStateKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBStateKind, Self::Err> {
        match input {
            "StateKind_UNDEFINED" => Ok(OpenFMBStateKind::StateKind_UNDEFINED),
            "StateKind_off" => Ok(OpenFMBStateKind::StateKind_off),
            "StateKind_on" => Ok(OpenFMBStateKind::StateKind_on),
            "StateKind_standby" => Ok(OpenFMBStateKind::StateKind_standby),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBStateKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBStateKind::StateKind_UNDEFINED as i32 => {
                Ok(OpenFMBStateKind::StateKind_UNDEFINED)
            }
            x if x == OpenFMBStateKind::StateKind_off as i32 => Ok(OpenFMBStateKind::StateKind_off),
            x if x == OpenFMBStateKind::StateKind_on as i32 => Ok(OpenFMBStateKind::StateKind_on),
            x if x == OpenFMBStateKind::StateKind_standby as i32 => {
                Ok(OpenFMBStateKind::StateKind_standby)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBAlrmKind {
    AlrmKind_ground_fault = 0,
    AlrmKind_dc_over_voltage = 1,
    AlrmKind_ac_disconnect_open = 2,
    AlrmKind_dc_disconnect_open = 3,
    AlrmKind_grid_disconnect = 4,
    AlrmKind_cabinet_open = 5,
    AlrmKind_manual_shutdown = 6,
    AlrmKind_over_temperature = 7,
    AlrmKind_frequency_above_limit = 8,
    AlrmKind_frequency_under_limit = 9,
    AlrmKind_ac_voltage_above_limit = 10,
    AlrmKind_ac_voltage_under_limit = 11,
    AlrmKind_blown_string_fuse_on_input = 12,
    AlrmKind_under_temperature = 13,
    AlrmKind_generic_memory_or_communication_error = 14,
    AlrmKind_hardware_test_failure = 15,
    AlrmKind_manufacturer_alarm = 16,
}
impl FromStr for OpenFMBAlrmKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBAlrmKind, Self::Err> {
        match input {
            "AlrmKind_ground_fault" => Ok(OpenFMBAlrmKind::AlrmKind_ground_fault),
            "AlrmKind_dc_over_voltage" => Ok(OpenFMBAlrmKind::AlrmKind_dc_over_voltage),
            "AlrmKind_ac_disconnect_open" => Ok(OpenFMBAlrmKind::AlrmKind_ac_disconnect_open),
            "AlrmKind_dc_disconnect_open" => Ok(OpenFMBAlrmKind::AlrmKind_dc_disconnect_open),
            "AlrmKind_grid_disconnect" => Ok(OpenFMBAlrmKind::AlrmKind_grid_disconnect),
            "AlrmKind_cabinet_open" => Ok(OpenFMBAlrmKind::AlrmKind_cabinet_open),
            "AlrmKind_manual_shutdown" => Ok(OpenFMBAlrmKind::AlrmKind_manual_shutdown),
            "AlrmKind_over_temperature" => Ok(OpenFMBAlrmKind::AlrmKind_over_temperature),
            "AlrmKind_frequency_above_limit" => Ok(OpenFMBAlrmKind::AlrmKind_frequency_above_limit),
            "AlrmKind_frequency_under_limit" => Ok(OpenFMBAlrmKind::AlrmKind_frequency_under_limit),
            "AlrmKind_ac_voltage_above_limit" => {
                Ok(OpenFMBAlrmKind::AlrmKind_ac_voltage_above_limit)
            }
            "AlrmKind_ac_voltage_under_limit" => {
                Ok(OpenFMBAlrmKind::AlrmKind_ac_voltage_under_limit)
            }
            "AlrmKind_blown_string_fuse_on_input" => {
                Ok(OpenFMBAlrmKind::AlrmKind_blown_string_fuse_on_input)
            }
            "AlrmKind_under_temperature" => Ok(OpenFMBAlrmKind::AlrmKind_under_temperature),
            "AlrmKind_generic_memory_or_communication_error" => {
                Ok(OpenFMBAlrmKind::AlrmKind_generic_memory_or_communication_error)
            }
            "AlrmKind_hardware_test_failure" => Ok(OpenFMBAlrmKind::AlrmKind_hardware_test_failure),
            "AlrmKind_manufacturer_alarm" => Ok(OpenFMBAlrmKind::AlrmKind_manufacturer_alarm),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBAlrmKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBAlrmKind::AlrmKind_ground_fault as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_ground_fault)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_dc_over_voltage as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_dc_over_voltage)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_ac_disconnect_open as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_ac_disconnect_open)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_dc_disconnect_open as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_dc_disconnect_open)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_grid_disconnect as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_grid_disconnect)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_cabinet_open as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_cabinet_open)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_manual_shutdown as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_manual_shutdown)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_over_temperature as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_over_temperature)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_frequency_above_limit as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_frequency_above_limit)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_frequency_under_limit as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_frequency_under_limit)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_ac_voltage_above_limit as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_ac_voltage_above_limit)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_ac_voltage_under_limit as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_ac_voltage_under_limit)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_blown_string_fuse_on_input as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_blown_string_fuse_on_input)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_under_temperature as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_under_temperature)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_generic_memory_or_communication_error as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_generic_memory_or_communication_error)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_hardware_test_failure as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_hardware_test_failure)
            }
            x if x == OpenFMBAlrmKind::AlrmKind_manufacturer_alarm as i32 => {
                Ok(OpenFMBAlrmKind::AlrmKind_manufacturer_alarm)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBGridConnectionStateKind {
    GridConnectionStateKind_disconnected = 0,
    GridConnectionStateKind_connected = 1,
}
impl FromStr for OpenFMBGridConnectionStateKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBGridConnectionStateKind, Self::Err> {
        match input {
            "GridConnectionStateKind_disconnected" => {
                Ok(OpenFMBGridConnectionStateKind::GridConnectionStateKind_disconnected)
            }
            "GridConnectionStateKind_connected" => {
                Ok(OpenFMBGridConnectionStateKind::GridConnectionStateKind_connected)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBGridConnectionStateKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x
                == OpenFMBGridConnectionStateKind::GridConnectionStateKind_disconnected as i32 =>
            {
                Ok(OpenFMBGridConnectionStateKind::GridConnectionStateKind_disconnected)
            }
            x if x == OpenFMBGridConnectionStateKind::GridConnectionStateKind_connected as i32 => {
                Ok(OpenFMBGridConnectionStateKind::GridConnectionStateKind_connected)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBOperatingStateKind {
    OperatingStateKind_UNDEFINED = 0,
    OperatingStateKind_off = 1,
    OperatingStateKind_disconnected_and_standby = 2,
    OperatingStateKind_disconnected_and_available = 3,
    OperatingStateKind_disconnected_and_authorized = 4,
    OperatingStateKind_starting_and_synchronizing = 5,
    OperatingStateKind_connected_and_idle = 6,
    OperatingStateKind_connected_and_generating = 7,
    OperatingStateKind_connected_and_consuming = 8,
    OperatingStateKind_stopping = 9,
    OperatingStateKind_disconnected_and_blocked = 10,
    OperatingStateKind_disconnected_and_in_maintenance = 11,
    OperatingStateKind_ceased_to_energize = 12,
    OperatingStateKind_failed = 13,
}
impl FromStr for OpenFMBOperatingStateKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBOperatingStateKind, Self::Err> {
        match input {
            "OperatingStateKind_UNDEFINED" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_UNDEFINED)
            }
            "OperatingStateKind_off" => Ok(OpenFMBOperatingStateKind::OperatingStateKind_off),
            "OperatingStateKind_disconnected_and_standby" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_standby)
            }
            "OperatingStateKind_disconnected_and_available" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_available)
            }
            "OperatingStateKind_disconnected_and_authorized" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_authorized)
            }
            "OperatingStateKind_starting_and_synchronizing" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_starting_and_synchronizing)
            }
            "OperatingStateKind_connected_and_idle" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_idle)
            }
            "OperatingStateKind_connected_and_generating" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_generating)
            }
            "OperatingStateKind_connected_and_consuming" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_consuming)
            }
            "OperatingStateKind_stopping" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_stopping)
            }
            "OperatingStateKind_disconnected_and_blocked" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_blocked)
            }
            "OperatingStateKind_disconnected_and_in_maintenance" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_in_maintenance)
            }
            "OperatingStateKind_ceased_to_energize" => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_ceased_to_energize)
            }
            "OperatingStateKind_failed" => Ok(OpenFMBOperatingStateKind::OperatingStateKind_failed),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBOperatingStateKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_UNDEFINED as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_UNDEFINED)
            }
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_off as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_off)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_standby
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_standby)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_available
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_available)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_authorized
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_authorized)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_starting_and_synchronizing
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_starting_and_synchronizing)
            }
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_connected_and_idle as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_idle)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_connected_and_generating
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_generating)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_connected_and_consuming as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_connected_and_consuming)
            }
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_stopping as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_stopping)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_blocked
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_blocked)
            }
            x if x
                == OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_in_maintenance
                    as i32 =>
            {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_disconnected_and_in_maintenance)
            }
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_ceased_to_energize as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_ceased_to_energize)
            }
            x if x == OpenFMBOperatingStateKind::OperatingStateKind_failed as i32 => {
                Ok(OpenFMBOperatingStateKind::OperatingStateKind_failed)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBDirectionModeKind {
    DirectionModeKind_UNDEFINED = 0,
    DirectionModeKind_locked_forward = 1,
    DirectionModeKind_locked_reverse = 2,
    DirectionModeKind_reverse_idle = 3,
    DirectionModeKind_bidirectional = 4,
    DirectionModeKind_neutral_idle = 5,
    DirectionModeKind_cogeneration = 6,
    DirectionModeKind_reactive_bidirectional = 7,
    DirectionModeKind_bias_bidirectional = 8,
    DirectionModeKind_bias_cogeneration = 9,
    DirectionModeKind_reverse_cogeneration = 10,
}
impl FromStr for OpenFMBDirectionModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBDirectionModeKind, Self::Err> {
        match input {
            "DirectionModeKind_UNDEFINED" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_UNDEFINED)
            }
            "DirectionModeKind_locked_forward" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_locked_forward)
            }
            "DirectionModeKind_locked_reverse" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_locked_reverse)
            }
            "DirectionModeKind_reverse_idle" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reverse_idle)
            }
            "DirectionModeKind_bidirectional" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bidirectional)
            }
            "DirectionModeKind_neutral_idle" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_neutral_idle)
            }
            "DirectionModeKind_cogeneration" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_cogeneration)
            }
            "DirectionModeKind_reactive_bidirectional" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reactive_bidirectional)
            }
            "DirectionModeKind_bias_bidirectional" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bias_bidirectional)
            }
            "DirectionModeKind_bias_cogeneration" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bias_cogeneration)
            }
            "DirectionModeKind_reverse_cogeneration" => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reverse_cogeneration)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBDirectionModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_UNDEFINED as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_UNDEFINED)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_locked_forward as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_locked_forward)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_locked_reverse as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_locked_reverse)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_reverse_idle as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reverse_idle)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_bidirectional as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bidirectional)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_neutral_idle as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_neutral_idle)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_cogeneration as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_cogeneration)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_reactive_bidirectional as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reactive_bidirectional)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_bias_bidirectional as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bias_bidirectional)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_bias_cogeneration as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_bias_cogeneration)
            }
            x if x == OpenFMBDirectionModeKind::DirectionModeKind_reverse_cogeneration as i32 => {
                Ok(OpenFMBDirectionModeKind::DirectionModeKind_reverse_cogeneration)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBVoltLimitModeKind {
    VoltLimitModeKind_UNDEFINED = 0,
    VoltLimitModeKind_off = 1,
    VoltLimitModeKind_high_limit_only = 2,
    VoltLimitModeKind_low_limit_only = 3,
    VoltLimitModeKind_high_low_limits = 4,
    VoltLimitModeKind_ivvc_high_limit_only = 5,
    VoltLimitModeKind_ivvc_low_limit_only = 6,
    VoltLimitModeKind_ivvc_high_low_limits = 7,
}
impl FromStr for OpenFMBVoltLimitModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBVoltLimitModeKind, Self::Err> {
        match input {
            "VoltLimitModeKind_UNDEFINED" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_UNDEFINED)
            }
            "VoltLimitModeKind_off" => Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_off),
            "VoltLimitModeKind_high_limit_only" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_limit_only)
            }
            "VoltLimitModeKind_low_limit_only" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_low_limit_only)
            }
            "VoltLimitModeKind_high_low_limits" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_low_limits)
            }
            "VoltLimitModeKind_ivvc_high_limit_only" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_limit_only)
            }
            "VoltLimitModeKind_ivvc_low_limit_only" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_low_limit_only)
            }
            "VoltLimitModeKind_ivvc_high_low_limits" => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_low_limits)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBVoltLimitModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_UNDEFINED as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_UNDEFINED)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_off as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_off)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_limit_only as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_limit_only)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_low_limit_only as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_low_limit_only)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_low_limits as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_high_low_limits)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_limit_only as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_limit_only)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_low_limit_only as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_low_limit_only)
            }
            x if x == OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_low_limits as i32 => {
                Ok(OpenFMBVoltLimitModeKind::VoltLimitModeKind_ivvc_high_low_limits)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBFaultDirectionKind {
    FaultDirectionKind_UNDEFINED = 0,
    FaultDirectionKind_unknown = 1,
    FaultDirectionKind_forward = 2,
    FaultDirectionKind_backward = 3,
    FaultDirectionKind_both = 4,
}
impl FromStr for OpenFMBFaultDirectionKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBFaultDirectionKind, Self::Err> {
        match input {
            "FaultDirectionKind_UNDEFINED" => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_UNDEFINED)
            }
            "FaultDirectionKind_unknown" => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_unknown)
            }
            "FaultDirectionKind_forward" => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_forward)
            }
            "FaultDirectionKind_backward" => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_backward)
            }
            "FaultDirectionKind_both" => Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_both),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBFaultDirectionKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBFaultDirectionKind::FaultDirectionKind_UNDEFINED as i32 => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_UNDEFINED)
            }
            x if x == OpenFMBFaultDirectionKind::FaultDirectionKind_unknown as i32 => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_unknown)
            }
            x if x == OpenFMBFaultDirectionKind::FaultDirectionKind_forward as i32 => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_forward)
            }
            x if x == OpenFMBFaultDirectionKind::FaultDirectionKind_backward as i32 => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_backward)
            }
            x if x == OpenFMBFaultDirectionKind::FaultDirectionKind_both as i32 => {
                Ok(OpenFMBFaultDirectionKind::FaultDirectionKind_both)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBPhaseFaultDirectionKind {
    PhaseFaultDirectionKind_UNDEFINED = 0,
    PhaseFaultDirectionKind_unknown = 1,
    PhaseFaultDirectionKind_forward = 2,
    PhaseFaultDirectionKind_backward = 3,
}
impl FromStr for OpenFMBPhaseFaultDirectionKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBPhaseFaultDirectionKind, Self::Err> {
        match input {
            "PhaseFaultDirectionKind_UNDEFINED" => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_UNDEFINED)
            }
            "PhaseFaultDirectionKind_unknown" => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_unknown)
            }
            "PhaseFaultDirectionKind_forward" => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_forward)
            }
            "PhaseFaultDirectionKind_backward" => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_backward)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBPhaseFaultDirectionKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_UNDEFINED as i32 => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_UNDEFINED)
            }
            x if x == OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_unknown as i32 => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_unknown)
            }
            x if x == OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_forward as i32 => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_forward)
            }
            x if x == OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_backward as i32 => {
                Ok(OpenFMBPhaseFaultDirectionKind::PhaseFaultDirectionKind_backward)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBRecloseActionKind {
    RecloseActionKind_UNDEFINED = 0,
    RecloseActionKind_idle = 1,
    RecloseActionKind_cycling = 2,
    RecloseActionKind_lockout = 3,
}
impl FromStr for OpenFMBRecloseActionKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBRecloseActionKind, Self::Err> {
        match input {
            "RecloseActionKind_UNDEFINED" => {
                Ok(OpenFMBRecloseActionKind::RecloseActionKind_UNDEFINED)
            }
            "RecloseActionKind_idle" => Ok(OpenFMBRecloseActionKind::RecloseActionKind_idle),
            "RecloseActionKind_cycling" => Ok(OpenFMBRecloseActionKind::RecloseActionKind_cycling),
            "RecloseActionKind_lockout" => Ok(OpenFMBRecloseActionKind::RecloseActionKind_lockout),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBRecloseActionKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBRecloseActionKind::RecloseActionKind_UNDEFINED as i32 => {
                Ok(OpenFMBRecloseActionKind::RecloseActionKind_UNDEFINED)
            }
            x if x == OpenFMBRecloseActionKind::RecloseActionKind_idle as i32 => {
                Ok(OpenFMBRecloseActionKind::RecloseActionKind_idle)
            }
            x if x == OpenFMBRecloseActionKind::RecloseActionKind_cycling as i32 => {
                Ok(OpenFMBRecloseActionKind::RecloseActionKind_cycling)
            }
            x if x == OpenFMBRecloseActionKind::RecloseActionKind_lockout as i32 => {
                Ok(OpenFMBRecloseActionKind::RecloseActionKind_lockout)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBReactivePowerControlKind {
    ReactivePowerControlKind_UNDEFINED = 0,
    ReactivePowerControlKind_advanced = 1,
    ReactivePowerControlKind_droop = 2,
    ReactivePowerControlKind_voltage = 3,
    ReactivePowerControlKind_reactivePower = 4,
    ReactivePowerControlKind_powerFactor = 5,
}
impl FromStr for OpenFMBReactivePowerControlKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBReactivePowerControlKind, Self::Err> {
        match input {
            "ReactivePowerControlKind_UNDEFINED" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_UNDEFINED)
            }
            "ReactivePowerControlKind_advanced" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_advanced)
            }
            "ReactivePowerControlKind_droop" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_droop)
            }
            "ReactivePowerControlKind_voltage" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_voltage)
            }
            "ReactivePowerControlKind_reactivePower" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_reactivePower)
            }
            "ReactivePowerControlKind_powerFactor" => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_powerFactor)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBReactivePowerControlKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x
                == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_UNDEFINED as i32 =>
            {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_UNDEFINED)
            }
            x if x == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_advanced as i32 => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_advanced)
            }
            x if x == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_droop as i32 => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_droop)
            }
            x if x == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_voltage as i32 => {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_voltage)
            }
            x if x
                == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_reactivePower
                    as i32 =>
            {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_reactivePower)
            }
            x if x
                == OpenFMBReactivePowerControlKind::ReactivePowerControlKind_powerFactor as i32 =>
            {
                Ok(OpenFMBReactivePowerControlKind::ReactivePowerControlKind_powerFactor)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBRealPowerControlKind {
    RealPowerControlKind_UNDEFINED = 0,
    RealPowerControlKind_advanced = 1,
    RealPowerControlKind_droop = 2,
    RealPowerControlKind_isochronous = 3,
    RealPowerControlKind_realPower = 4,
}
impl FromStr for OpenFMBRealPowerControlKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBRealPowerControlKind, Self::Err> {
        match input {
            "RealPowerControlKind_UNDEFINED" => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_UNDEFINED)
            }
            "RealPowerControlKind_advanced" => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_advanced)
            }
            "RealPowerControlKind_droop" => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_droop)
            }
            "RealPowerControlKind_isochronous" => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_isochronous)
            }
            "RealPowerControlKind_realPower" => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_realPower)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBRealPowerControlKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBRealPowerControlKind::RealPowerControlKind_UNDEFINED as i32 => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_UNDEFINED)
            }
            x if x == OpenFMBRealPowerControlKind::RealPowerControlKind_advanced as i32 => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_advanced)
            }
            x if x == OpenFMBRealPowerControlKind::RealPowerControlKind_droop as i32 => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_droop)
            }
            x if x == OpenFMBRealPowerControlKind::RealPowerControlKind_isochronous as i32 => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_isochronous)
            }
            x if x == OpenFMBRealPowerControlKind::RealPowerControlKind_realPower as i32 => {
                Ok(OpenFMBRealPowerControlKind::RealPowerControlKind_realPower)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBCircuitSegmentServiceModeKind {
    CircuitSegmentServiceModeKind_UNDEFINED = 0,
    CircuitSegmentServiceModeKind_none = 1,
    CircuitSegmentServiceModeKind_auto = 2,
    CircuitSegmentServiceModeKind_manual = 3,
    CircuitSegmentServiceModeKind_netzero = 4,
    CircuitSegmentServiceModeKind_start = 5,
    CircuitSegmentServiceModeKind_stop = 6,
}
impl FromStr for OpenFMBCircuitSegmentServiceModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBCircuitSegmentServiceModeKind, Self::Err> {
        match input {
            "CircuitSegmentServiceModeKind_UNDEFINED" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_UNDEFINED)
            }
            "CircuitSegmentServiceModeKind_none" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_none)
            }
            "CircuitSegmentServiceModeKind_auto" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_auto)
            }
            "CircuitSegmentServiceModeKind_manual" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_manual)
            }
            "CircuitSegmentServiceModeKind_netzero" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_netzero)
            }
            "CircuitSegmentServiceModeKind_start" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_start)
            }
            "CircuitSegmentServiceModeKind_stop" => {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_stop)
            }
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBCircuitSegmentServiceModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_UNDEFINED
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_UNDEFINED)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_none
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_none)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_auto
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_auto)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_manual
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_manual)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_netzero
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_netzero)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_start
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_start)
            }
            x if x
                == OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_stop
                    as i32 =>
            {
                Ok(OpenFMBCircuitSegmentServiceModeKind::CircuitSegmentServiceModeKind_stop)
            }
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum OpenFMBControlModeKind {
    ControlModeKind_UNDEFINED = 0,
    ControlModeKind_auto = 1,
    ControlModeKind_manual = 2,
    ControlModeKind_override = 3,
    ControlModeKind_remote = 4,
}
impl FromStr for OpenFMBControlModeKind {
    type Err = ();
    fn from_str(input: &str) -> Result<OpenFMBControlModeKind, Self::Err> {
        match input {
            "ControlModeKind_UNDEFINED" => Ok(OpenFMBControlModeKind::ControlModeKind_UNDEFINED),
            "ControlModeKind_auto" => Ok(OpenFMBControlModeKind::ControlModeKind_auto),
            "ControlModeKind_manual" => Ok(OpenFMBControlModeKind::ControlModeKind_manual),
            "ControlModeKind_override" => Ok(OpenFMBControlModeKind::ControlModeKind_override),
            "ControlModeKind_remote" => Ok(OpenFMBControlModeKind::ControlModeKind_remote),
            _ => Err(()),
        }
    }
}
impl TryFrom<i32> for OpenFMBControlModeKind {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == OpenFMBControlModeKind::ControlModeKind_UNDEFINED as i32 => {
                Ok(OpenFMBControlModeKind::ControlModeKind_UNDEFINED)
            }
            x if x == OpenFMBControlModeKind::ControlModeKind_auto as i32 => {
                Ok(OpenFMBControlModeKind::ControlModeKind_auto)
            }
            x if x == OpenFMBControlModeKind::ControlModeKind_manual as i32 => {
                Ok(OpenFMBControlModeKind::ControlModeKind_manual)
            }
            x if x == OpenFMBControlModeKind::ControlModeKind_override as i32 => {
                Ok(OpenFMBControlModeKind::ControlModeKind_override)
            }
            x if x == OpenFMBControlModeKind::ControlModeKind_remote as i32 => {
                Ok(OpenFMBControlModeKind::ControlModeKind_remote)
            }
            _ => Err(()),
        }
    }
}
