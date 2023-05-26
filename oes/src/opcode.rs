// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

/// OpCode
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OpCode {
    SR,
    DO,
    TC,
    RM,
    TT,
}

impl OpCode {
    pub fn as_str(&self) -> &str {
        match self {
            OpCode::SR => "SR",
            OpCode::DO => "DO",
            OpCode::TC => "TC",
            OpCode::RM => "RM",
            OpCode::TT => "TT",
        }
    }
}

impl std::str::FromStr for OpCode {
    type Err = ();
    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        match input {
            "SR" => Ok(OpCode::SR),
            "DO" => Ok(OpCode::DO),
            "TC" => Ok(OpCode::TC),
            "RM" => Ok(OpCode::RM),
            "TT" => Ok(OpCode::TT),
            _ => Err(()),
        }
    }
}
