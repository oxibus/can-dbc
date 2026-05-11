use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{collect_node_names, next_rule, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<String>,
}

impl TryFrom<Pair<'_, Rule>> for MessageTransmitter {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::message_transmitter)?;

        Ok(Self {
            message_id: next_rule(&mut pairs, Rule::message_id)?.try_into()?,
            transmitter: collect_node_names(&mut pairs, Rule::transmitter)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn message_transmitters_test() {
        let def = "
BO_TX_BU_ 12345 : XZY,ABC;
";
        let exp = MessageTransmitter {
            message_id: MessageId::Standard(12345),
            transmitter: vec!["XZY".to_string(), "ABC".to_string()],
        };
        let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
        assert_eq!(val, exp);

        // Same as above, but without space before the colon
        let def = "
BO_TX_BU_ 12345 :XZY,ABC;
";
        let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
        assert_eq!(val, exp);

        let def = "
BO_TX_BU_ 12345 : Vector__XXX;
";
        let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
        assert_eq!(val.transmitter, Vec::<String>::new());
    }
}
