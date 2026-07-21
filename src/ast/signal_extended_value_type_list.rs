use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, SignalExtendedValueType};
use crate::parser::{expect_empty, next, next_rule, next_string, validated_inner, DbcError};

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
        let value = Self {
            message_id: next_rule(&mut pairs, Rule::message_id)?.try_into()?,
            signal_name: next_string(&mut pairs, Rule::signal_name)?,
            signal_extended_value_type: next(&mut pairs)?.as_rule().try_into()?,
        };
        expect_empty(&pairs)?;

        Ok(value)
    }
}
