use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, SignalExtendedValueType};
use crate::parser::{expect_empty, next_rule, next_string, parse_uint, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalExtendedValueTypeList {
    pub message_id: MessageId,
    pub signal_name: String,
    pub signal_extended_value_type: SignalExtendedValueType,
}

impl TryFrom<Pair<'_, Rule>> for SignalExtendedValueTypeList {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::signal_value_type)?;

        let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
        let signal_name = next_string(&mut pairs, Rule::signal_name)?;
        let value_type = parse_uint(next_rule(&mut pairs, Rule::int)?)?;
        expect_empty(&pairs)?;

        Ok(Self {
            message_id,
            signal_name,
            signal_extended_value_type: value_type.try_into()?,
        })
    }
}
