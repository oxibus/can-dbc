use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, SignalExtendedValueType};
use crate::parser;
use crate::parser::{expect_empty, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalExtendedValueTypeList {
    pub message_id: MessageId,
    pub signal_name: String,
    pub signal_extended_value_type: SignalExtendedValueType,
}

impl SignalExtendedValueTypeList {
    /// Parse signal value type: `SIG_VALTYPE_ message_id signal_name : value_type;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<SignalExtendedValueTypeList> {
        let mut pairs = pair.into_inner();

        let message_id =
            parser::parse_uint(parser::next_rule(&mut pairs, Rule::message_id)?)? as u32;
        let signal_name = parser::next_rule(&mut pairs, Rule::signal_name)?
            .as_str()
            .to_string();
        let value_type = parser::parse_uint(parser::next_rule(&mut pairs, Rule::int)?)?;
        expect_empty(&mut pairs)?;

        let msg_id = if message_id & (1 << 31) != 0 {
            MessageId::Extended(message_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(message_id as u16)
        };

        let signal_extended_value_type = match value_type {
            0 => SignalExtendedValueType::SignedOrUnsignedInteger,
            1 => SignalExtendedValueType::IEEEfloat32Bit,
            2 => SignalExtendedValueType::IEEEdouble64bit,
            v => panic!("Unknown signal extended value type: {v}"),
        };

        Ok(SignalExtendedValueTypeList {
            message_id: msg_id,
            signal_name,
            signal_extended_value_type,
        })
    }
}
