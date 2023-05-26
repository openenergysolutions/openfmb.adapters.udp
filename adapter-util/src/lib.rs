#![allow(dead_code)]
#![allow(unused)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use chrono::format::ParseError;
use chrono::{DateTime, NaiveDateTime, Utc};
use openfmb::messages::commonmodule::*;
use std::any::Any;
use std::fmt::Debug;
use uuid::Uuid;

use log::{debug, error};
use prost::Message;
use serde_yaml::Value;

pub mod visitors;
pub use visitors::*;

pub mod configuration;
pub use configuration::*;

pub mod adapter_config;
pub use adapter_config::*;

pub mod utils;
pub use utils::*;

macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

pub trait Visitor: Sized {
    type MessageType: prost::Message;
    fn visit(&mut self, t: &mut Self::MessageType);
}

pub trait Setter<P: Message, V: Any + Debug> {
    fn execute(&self, p: &mut P, value: V);
}

pub trait Getter<P: Message> {
    fn execute(&self, p: &mut P) -> Option<Command> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct CommandTimestamp {
    pub seconds: u64,
}

impl CommandTimestamp {
    pub fn new(seconds: u64) -> CommandTimestamp {
        CommandTimestamp { seconds: seconds }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    StringValue(String, Vec<ControlMappingOutput>, Option<CommandTimestamp>),
    BoolValue(bool, Vec<ControlMappingOutput>, Option<CommandTimestamp>),
    RealValue(f64, Vec<ControlMappingOutput>, Option<CommandTimestamp>),
    IntValue(i64, Vec<ControlMappingOutput>, Option<CommandTimestamp>),
}

pub trait ProfileMapping {
    fn profile_name(&mut self) -> String;
}

pub fn new_uuid() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

pub fn timestamp_from_datetime(t: DateTime<Utc>) -> Timestamp {
    Timestamp {
        nanoseconds: (ms_to_fraction(t.timestamp_subsec_millis()) as u32),
        seconds: t.timestamp() as u64,
        tq: None,
    }
}

pub fn timestamp_from_f64(val: f64) -> Timestamp {
    // TODO
    Timestamp {
        nanoseconds: 0,
        seconds: val as u64,
        tq: None,
    }
}

pub fn get_current_timestamp() -> Timestamp {
    timestamp_from_datetime(Utc::now())
}

pub fn fraction_to_ms(fraction: u32) -> u32 {
    (fraction as f64 / 1000f64 * ((2 ^ 32) as f64)) as u32
}

pub fn ms_to_fraction(ms: u32) -> u32 {
    ((ms as f64) * 1000f64 / (2 ^ 32) as f64) as u32
}

pub struct Builder {}
impl VisitorBuilder for Builder {}

pub trait ConfigReadVisitor<T: Message> {
    // Update
    fn update_boolean(&mut self, _key: &str, _t: &mut T, _v: bool) {
        debug!("ConfigReadVisitor:: default behavior to handle boolean");
    }
    fn update_i32(&mut self, _key: &str, _t: &mut T, _v: i32) {
        debug!("ConfigReadVisitor:: default behavior to handle i32");
    }
    fn update_i64(&mut self, _key: &str, _t: &mut T, _v: i64) {
        debug!("ConfigReadVisitor:: default behavior to handle i64");
    }
    fn update_f32(&mut self, _key: &str, _t: &mut T, _v: f32) {
        debug!("ConfigReadVisitor:: default behavior to handle f32");
    }
    fn update_f64(&mut self, _key: &str, _t: &mut T, _v: f64) {
        debug!("ConfigReadVisitor:: default behavior to handle f64");
    }
    fn update_quality(&mut self, _key: &str, _t: &mut T, _q: Quality) {
        debug!("ConfigReadVisitor:: default behavior to handle quality");
    }
    fn update_timestamp(&mut self, _key: &str, _t: &mut T, _timestamp: Timestamp) {
        debug!("ConfigReadVisitor:: default behavior to handle timestamp");
    }
    fn update_string(&mut self, _key: &str, _t: &mut T, _v: String) {
        debug!("ConfigReadVisitor:: default behavior to handle string");
    }

    // Read
    fn get_boolean(&mut self, _key: &str, _t: &mut T) -> Option<bool> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitString {
    bits: Vec<char>,
}

impl BitString {
    fn from_str(s: &str) -> BitString {
        BitString {
            bits: s
                .to_string()
                .replace("[", "")
                .replace("]", "")
                .chars()
                .collect(),
        }
    }

    fn is_bit_on(&self, bit_num: usize) -> bool {
        if self.bits.len() <= bit_num {
            return false;
        }

        match self.bits[bit_num].to_digit(2) {
            Some(d) => {
                if d != 0 {
                    return true;
                }
            }
            None => {
                return false;
            }
        }
        return false;
    }

    fn to_qual(&self) -> Quality {
        let mut qual = Quality::default();
        if !self.is_bit_on(0) && self.is_bit_on(1) {
            qual.validity = 2; // ValidityKind::ValidityKind_invalid;
        } else if self.is_bit_on(0) && self.is_bit_on(1) {
            qual.validity = 4; // ValidityKind::ValidityKind_questionable;
        } else if self.is_bit_on(0) && !self.is_bit_on(1) {
            qual.validity = 3; // ValidityKind::ValidityKind_reserved;
        }

        if self.is_bit_on(12) {
            qual.operator_blocked = true;
        }
        if self.is_bit_on(11) {
            qual.test = true
        }
        let mut detail_qual = DetailQual::default();
        let mut detail_qual_set = false;

        if self.is_bit_on(10) {
            qual.source = 1; // SourceKind::SourceKind_substituted;
        }
        if self.is_bit_on(9) {
            detail_qual.inaccurate = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(8) {
            detail_qual.inconsistent = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(7) {
            detail_qual.old_data = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(6) {
            detail_qual.failure = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(5) {
            detail_qual.oscillatory = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(4) {
            detail_qual.bad_reference = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(3) {
            detail_qual.out_of_range = true;
            detail_qual_set = true;
        }
        if self.is_bit_on(2) {
            detail_qual.overflow = true;
            detail_qual_set = true;
        }

        if detail_qual_set {
            qual.detail_qual = Some(detail_qual);
        }

        return qual;
    }

    pub fn to_quality(s: &str) -> Quality {
        let bs = BitString::from_str(s);

        bs.to_qual()
    }

    pub fn to_time_quality(s: &str) -> TimeQuality {
        let bs = BitString::from_str(s);

        let mut tq = TimeQuality::default();
        if bs.is_bit_on(0) {
            tq.leap_seconds_known = true;
        }
        if bs.is_bit_on(1) {
            tq.clock_failure = true;
        }
        if bs.is_bit_on(2) {
            tq.clock_not_synchronized = true;
        }

        let mut ss: String = bs.bits.into_iter().collect();
        if ss.len() == 8 {
            ss = ss.chars().skip(3).collect();

            match parse_bit_string(&ss) {
                Some(u) => tq.time_accuracy = u,
                _ => {
                    error!("Invalid Time accuracy bitstring: {}", ss);
                }
            }
        }
        tq
    }
}

pub fn parse_utc_time(s: &str) -> Option<Timestamp> {
    let mut tokens = s.split(",");
    let vec: Vec<&str> = tokens.collect();

    if vec.len() == 2 {
        let tq = BitString::to_time_quality(vec[1].trim());

        // sample format: 04/10/2021_19:34:06.582
        match NaiveDateTime::parse_from_str(vec[0].trim(), "%m/%d/%Y_%H:%M:%S.%f") {
            Ok(dt) => {
                let dt = DateTime::<Utc>::from_utc(dt, Utc);
                let mut timestamp = timestamp_from_datetime(dt);
                timestamp.tq = Some(tq);
                return Some(timestamp);
            }
            Err(e) => {
                error!("Error parsing time: {}", vec[0].trim());
                return None;
            }
        }
    }

    None
}
