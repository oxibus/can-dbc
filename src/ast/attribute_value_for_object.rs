use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValuedForObjectType;
use crate::parser::{
    expect_empty, inner_str, next_optional_rule, next_rule, next_string, parse_float, single_inner,
    single_inner_str, validated_inner, DbcError,
};
use crate::{AttributeValue, MessageId};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueForObject {
    type Error = DbcError;

    /// Parse attribute value: `BA_ attribute_name [object_type] object_name value;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        // 1) Validate wrapper and get inner pairs iterator
        let mut pairs = validated_inner(value, Rule::attr_value)?;

        // 2) Read attribute name (quoted_str -> inner string)
        let name = inner_str(next_rule(&mut pairs, Rule::attribute_name)?);

        // 3) Optionally parse an object specifier (at most one)
        // Try each expected object-rule in sequence using next_optional_rule
        let mut object_type = None;
        let mut message_id: Option<MessageId> = None;
        let mut signal_name: Option<String> = None;
        let mut node_name: Option<String> = None;
        let mut env_var_name: Option<String> = None;

        if let Some(node_var_pair) = next_optional_rule(&mut pairs, Rule::node_var)? {
            object_type = Some(Rule::node_var);
            node_name = Some(single_inner_str(node_var_pair, Rule::node_name)?);
        } else if let Some(msg_var_pair) = next_optional_rule(&mut pairs, Rule::msg_var)? {
            object_type = Some(Rule::msg_var);
            message_id = Some(single_inner(msg_var_pair, Rule::message_id)?.try_into()?);
        } else if let Some(signal_var_pair) = next_optional_rule(&mut pairs, Rule::signal_var)? {
            object_type = Some(Rule::signal_var);
            let mut inner = signal_var_pair.into_inner();
            message_id = Some(next_rule(&mut inner, Rule::message_id)?.try_into()?);
            signal_name = Some(next_string(&mut inner, Rule::ident)?);
            expect_empty(&inner)?;
        } else if let Some(env_var_pair) = next_optional_rule(&mut pairs, Rule::env_var)? {
            object_type = Some(Rule::env_var);
            let v = single_inner(env_var_pair, Rule::env_var_name)?;
            env_var_name = Some(v.as_str().to_string());
        }

        // Parse the value (either quoted_str or number). If missing, default to empty string.
        let value = if let Some(pair) = pairs.next() {
            match pair.as_rule() {
                Rule::quoted_str => AttributeValue::String(inner_str(pair)),
                Rule::number => AttributeValue::Double(parse_float(pair)?),
                _ => return Err(DbcError::ParseError),
            }
        } else {
            AttributeValue::String(String::new())
        };

        expect_empty(&pairs)?;

        // Determine attribute value type based on parsed components
        let value = match object_type {
            Some(Rule::signal_var) => {
                if let (Some(msg_id), Some(sig_name)) = (message_id, signal_name) {
                    AttributeValuedForObjectType::Signal(msg_id, sig_name, value)
                } else {
                    todo!()
                    // AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::msg_var) => {
                if let Some(msg_id) = message_id {
                    AttributeValuedForObjectType::MessageDefinition(msg_id, Some(value))
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::node_var) => {
                if let Some(node) = node_name {
                    AttributeValuedForObjectType::NetworkNode(node, value)
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::env_var) => {
                if let Some(env_var) = env_var_name {
                    AttributeValuedForObjectType::EnvVariable(env_var, value)
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            _ => AttributeValuedForObjectType::Raw(value),
        };

        Ok(Self { name, value })
    }
}
