use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, SignalExtendedValueType};
use crate::parser::{expect_empty, next_rule, next_string, parse_uint, DbcResult};

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

        let message_id = parse_uint(next_rule(&mut pairs, Rule::message_id)?)? as u32;
        let signal_name = next_string(&mut pairs, Rule::signal_name)?;
        let value_type = parse_uint(next_rule(&mut pairs, Rule::int)?)?;
        expect_empty(&mut pairs)?;

        let signal_extended_value_type = match value_type {
            0 => SignalExtendedValueType::SignedOrUnsignedInteger,
            1 => SignalExtendedValueType::IEEEfloat32Bit,
            2 => SignalExtendedValueType::IEEEdouble64bit,
            v => panic!("Unknown signal extended value type: {v}"),
        };

        Ok(SignalExtendedValueTypeList {
            message_id: MessageId::parse(message_id),
            signal_name,
            signal_extended_value_type,
        })
    }
}
