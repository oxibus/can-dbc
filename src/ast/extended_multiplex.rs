use can_dbc_pest::{Pair, Rule};

use crate::ast::{ExtendedMultiplexMapping, MessageId};
use crate::parser::{collect_all, next_rule, next_string, validated_inner, DbcError};

/// Mapping between multiplexors and multiplexed signals
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplex {
    pub message_id: MessageId,
    pub signal_name: String,
    pub multiplexor_signal_name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub mappings: Vec<ExtendedMultiplexMapping>,
}

impl TryFrom<Pair<'_, Rule>> for ExtendedMultiplex {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::sg_mul_val)?;

        let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
        let signal_name = next_string(&mut pairs, Rule::signal_name)?;
        let multiplexor_signal_name = next_string(&mut pairs, Rule::multiplexer_name)?;

        // Collect all remaining value pairs
        let mappings: Vec<ExtendedMultiplexMapping> =
            collect_all::<ExtendedMultiplexMapping>(&mut pairs)?;

        Ok(Self {
            message_id,
            signal_name,
            multiplexor_signal_name,
            mappings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn extended_multiplex_test() {
        // simple mapping
        let def = "
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1-1;
";
        let exp = ExtendedMultiplex {
            message_id: MessageId::Extended(2),
            signal_name: "muxed_A_1".to_string(),
            multiplexor_signal_name: "MUX_A".to_string(),
            mappings: vec![ExtendedMultiplexMapping {
                min_value: 1,
                max_value: 1,
            }],
        };
        let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
        assert_eq!(val, exp);
    }

    #[test]
    fn extended_multiplex_range_test() {
        // range mapping
        let def = "
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1568-2568;
";
        let exp = ExtendedMultiplex {
            message_id: MessageId::Extended(2),
            signal_name: "muxed_A_1".to_string(),
            multiplexor_signal_name: "MUX_A".to_string(),
            mappings: vec![ExtendedMultiplexMapping {
                min_value: 1568,
                max_value: 2568,
            }],
        };
        let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
        assert_eq!(val, exp);
    }

    #[test]
    fn extended_multiplex_mult_test() {
        // multiple mappings
        let def = "
SG_MUL_VAL_ 2147483650 muxed_B_5 MUX_B 5-5, 16-24;
";
        let exp = ExtendedMultiplex {
            message_id: MessageId::Extended(2),
            signal_name: "muxed_B_5".to_string(),
            multiplexor_signal_name: "MUX_B".to_string(),
            mappings: vec![
                ExtendedMultiplexMapping {
                    min_value: 5,
                    max_value: 5,
                },
                ExtendedMultiplexMapping {
                    min_value: 16,
                    max_value: 24,
                },
            ],
        };
        let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
        assert_eq!(val, exp);
    }
}
