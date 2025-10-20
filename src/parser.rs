//!
//! Parser module for DBC files using pest
//!

use std::str;

use can_dbc_pest::{DbcParser, Pair, Pairs, Parser as _, Rule};

use crate::{
    AccessNode, AccessType, AttributeDefault, AttributeDefinition, AttributeValue,
    AttributeValueForObject, AttributeValuedForObjectType, Baudrate, ByteOrder, Comment, Dbc,
    DbcError, DbcResult, EnvType, EnvironmentVariable, EnvironmentVariableData, ExtendedMultiplex,
    ExtendedMultiplexMapping, Message, MessageId, MessageTransmitter, MultiplexIndicator, Node,
    Signal, SignalExtendedValueType, SignalExtendedValueTypeList, SignalGroups, SignalType,
    SignalTypeRef, Symbol, Transmitter, ValDescription, ValueDescription, ValueTable, ValueType,
    Version,
};

/// Helper function to extract string content from quoted_str rule
fn parse_str(pair: Pair<Rule>) -> String {
    if pair.as_rule() == Rule::string {
        return pair.as_str().to_string();
    }
    for pair2 in pair.into_inner() {
        if pair2.as_rule() == Rule::string {
            return pair2.as_str().to_string();
        }
    }
    String::new()
}

/// Helper function to parse an integer from a pest pair
pub(crate) fn parse_int(pair: Pair<Rule>) -> DbcResult<i64> {
    pair.as_str()
        .parse::<i64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Helper function to parse an unsigned integer from a pest pair
pub(crate) fn parse_uint(pair: Pair<Rule>) -> DbcResult<u64> {
    pair.as_str()
        .parse::<u64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Helper function to parse a float from a pest pair
pub(crate) fn parse_float(pair: Pair<Rule>) -> DbcResult<f64> {
    pair.as_str()
        .parse::<f64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Parse version: VERSION "string"
pub(crate) fn parse_version(pair: Pair<Rule>) -> DbcResult<Version> {
    let mut inner_pairs = pair.into_inner();
    let version_str = parse_str(next_rule(&mut inner_pairs, Rule::quoted_str)?);
    // Don't use expect_empty here as there might be comments or whitespace
    Ok(Version(version_str))
}

/// Parse new symbols: NS_ : symbol1 symbol2 ...
pub(crate) fn parse_new_symbols(pair: Pair<Rule>) -> DbcResult<Vec<Symbol>> {
    let mut symbols = Vec::new();
    for pair2 in pair.into_inner() {
        if let Rule::ident = pair2.as_rule() {
            symbols.push(Symbol(pair2.as_str().to_string()));
        }
    }
    Ok(symbols)
}

/// Parse bit timing: BS_: [baud_rate : BTR1 , BTR2 ]
pub(crate) fn parse_bit_timing(pair: Pair<Rule>) -> DbcResult<Vec<Baudrate>> {
    let baudrates = Vec::new();
    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            other => panic!("What is this? {other:?}"),
        }
    }
    Ok(baudrates)
}

/// Parse nodes: BU_: node1 node2 node3 ...
pub(crate) fn parse_nodes(pair: Pair<Rule>) -> DbcResult<Vec<Node>> {
    let mut nodes = Vec::new();

    for pair2 in pair.into_inner() {
        if let Rule::node_name = pair2.as_rule() {
            nodes.push(Node(pair2.as_str().to_string()));
        }
    }

    Ok(nodes)
}

/// Parse message: BO_ message_id message_name: message_size transmitter
pub(crate) fn parse_message(pair: Pair<Rule>) -> DbcResult<Message> {
    let mut inner_pairs = pair.into_inner();

    // Parse msg_var (contains msg_literal ~ message_id)
    let msg_var_pair = next_rule(&mut inner_pairs, Rule::msg_var)?;
    let mut message_id = 0u32;
    for sub_pair in msg_var_pair.into_inner() {
        if sub_pair.as_rule() == Rule::message_id {
            message_id = parse_uint(sub_pair)? as u32;
        }
    }

    let message_name = next_rule(&mut inner_pairs, Rule::message_name)?
        .as_str()
        .to_string();
    let message_size = parse_uint(next_rule(&mut inner_pairs, Rule::message_size)?)?;
    let transmitter = next_rule(&mut inner_pairs, Rule::transmitter)?
        .as_str()
        .to_string();

    // Don't use expect_empty here as there might be comments or whitespace

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    let transmitter =
        if transmitter == "Vector__XXX" || transmitter == "VectorXXX" || transmitter.is_empty() {
            Transmitter::VectorXXX
        } else {
            Transmitter::NodeName(transmitter)
        };

    Ok(Message {
        id: msg_id,
        name: message_name,
        size: message_size,
        transmitter,
        signals: Vec::new(), // Signals will be parsed separately and associated later
    })
}

/// Parse comment: CM_ [BU_|BO_|SG_|EV_] object_name "comment_text";
pub(crate) fn parse_comment(pair: Pair<Rule>) -> DbcResult<Option<Comment>> {
    let mut comment_text = String::new();
    let mut message_id = None;
    let mut signal_name = None;
    let mut node_name = None;
    let mut env_var_name = None;

    // Process all inner pairs to extract information
    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::quoted_str => comment_text = parse_str(pair2),
            Rule::msg_var => {
                // msg_var contains msg_literal ~ message_id
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::message_id {
                        message_id = Some(parse_uint(sub_pair)? as u32);
                    }
                }
            }
            Rule::node_var => {
                // node_var contains node_literal ~ node_name
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::node_name {
                        node_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            Rule::env_var => {
                // env_var contains env_literal ~ env_var_name
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::env_var_name {
                        env_var_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            Rule::signal_var => {
                // signal_var contains signal_literal ~ message_id ~ ident
                for sub_pair in pair2.into_inner() {
                    match sub_pair.as_rule() {
                        Rule::message_id => {
                            message_id = Some(parse_uint(sub_pair)? as u32);
                        }
                        Rule::ident => {
                            signal_name = Some(sub_pair.as_str().to_string());
                        }
                        _ => {}
                    }
                }
            }
            Rule::message_id => message_id = Some(parse_uint(pair2)? as u32),
            Rule::signal_name => signal_name = Some(pair2.as_str().to_string()),
            Rule::node_name => node_name = Some(pair2.as_str().to_string()),
            Rule::env_var_name => env_var_name = Some(pair2.as_str().to_string()),
            other => panic!("What is this? {other:?}"),
        }
    }

    // Determine comment type based on parsed components
    // The grammar structure tells us:
    // - If we have message_id + signal_name => Signal comment
    // - If we have message_id only => Message comment
    // - If we have node_name => Node comment
    // - If we have env_var_name => Environment variable comment
    // - Otherwise => Plain comment

    if let Some(msg_id) = message_id {
        let msg_id = if msg_id & (1 << 31) != 0 {
            MessageId::Extended(msg_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(msg_id as u16)
        };

        return if let Some(sig_name) = signal_name {
            // Signal comment: CM_ SG_ message_id signal_name "comment"
            Ok(Some(Comment::Signal {
                message_id: msg_id,
                name: sig_name,
                comment: comment_text,
            }))
        } else {
            // Message comment: CM_ BO_ message_id "comment"
            Ok(Some(Comment::Message {
                id: msg_id,
                comment: comment_text,
            }))
        };
    } else if let Some(node) = node_name {
        // Node comment: CM_ BU_ node_name "comment"
        return Ok(Some(Comment::Node {
            name: node,
            comment: comment_text,
        }));
    } else if let Some(env_var) = env_var_name {
        // Environment variable comment: CM_ EV_ env_var_name "comment"
        return Ok(Some(Comment::EnvVar {
            name: env_var,
            comment: comment_text,
        }));
    } else if !comment_text.is_empty() {
        // Plain comment: CM_ "comment"
        return Ok(Some(Comment::Plain {
            comment: comment_text,
        }));
    }

    Ok(None)
}

/// Parse attribute definition: BA_DEF_ [object_type] attribute_name attribute_type [min max];
pub(crate) fn parse_attribute_definition(pair: Pair<Rule>) -> DbcResult<AttributeDefinition> {
    let mut definition_string = String::new();
    let mut object_type = None;

    // Collect all tokens to build the full definition string
    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::object_type => {
                // This is the new rule that captures the object type
                let text = pair2.as_str();
                if text == "SG_" {
                    object_type = Some("signal");
                } else if text == "BO_" {
                    object_type = Some("message");
                } else if text == "BU_" {
                    object_type = Some("node");
                } else if text == "EV_" {
                    object_type = Some("environment_variable");
                }
            }
            Rule::attribute_name
            | Rule::attribute_type_int
            | Rule::attribute_type_hex
            | Rule::attribute_type_float
            | Rule::attribute_type_string
            | Rule::attribute_type_enum => {
                if !definition_string.is_empty() {
                    definition_string.push(' ');
                }
                definition_string.push_str(pair2.as_str());
            }
            other => panic!("What is this? {other:?}"),
        }
    }

    // Return appropriate attribute definition based on object type
    match object_type {
        Some("signal") => Ok(AttributeDefinition::Signal(definition_string)),
        Some("message") => Ok(AttributeDefinition::Message(definition_string)),
        Some("node") => Ok(AttributeDefinition::Node(definition_string)),
        Some("environment_variable") => {
            Ok(AttributeDefinition::EnvironmentVariable(definition_string))
        }
        _ => Ok(AttributeDefinition::Plain(definition_string)),
    }
}

/// Parse attribute value: BA_ attribute_name [object_type] object_name value;
pub(crate) fn parse_attribute_value(pair: Pair<Rule>) -> DbcResult<AttributeValueForObject> {
    let mut attribute_name = String::new();
    let mut object_type = None;
    let mut message_id = None;
    let mut signal_name = None;
    let mut node_name = None;
    let mut env_var_name = None;
    let mut value = None;

    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::attribute_name => {
                attribute_name = parse_str(pair2);
            }
            // num_str_value is a silent rule, so we get quoted_str or number directly
            Rule::quoted_str => {
                value = Some(AttributeValue::String(parse_str(pair2)));
            }
            Rule::number => {
                value = Some(AttributeValue::Double(parse_float(pair2)?));
            }
            Rule::node_var => {
                object_type = Some("node");
                // Parse the node name from the inner pairs
                // node_var contains: node_literal ~ node_name
                // node_literal is silent (_), so we get node_name directly
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::node_name {
                        node_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            Rule::msg_var => {
                object_type = Some("message");
                // Parse the message ID from the inner pairs
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::message_id {
                        message_id = Some(parse_uint(sub_pair)? as u32);
                    }
                }
            }
            Rule::signal_var => {
                object_type = Some("signal");
                // Parse the message ID and signal name from the inner pairs
                for sub_pair in pair2.into_inner() {
                    match sub_pair.as_rule() {
                        Rule::message_id => {
                            message_id = Some(parse_uint(sub_pair)? as u32);
                        }
                        Rule::ident => {
                            signal_name = Some(sub_pair.as_str().to_string());
                        }
                        other => panic!("What is this? {other:?}"),
                    }
                }
            }
            Rule::env_var => {
                object_type = Some("env_var");
                // Parse the environment variable name from the inner pairs
                // env_var contains: env_literal ~ env_var_name
                // env_literal is silent (_), so we get env_var_name directly
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::env_var_name {
                        env_var_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            other => panic!("What is this? {other:?}"),
        }
    }

    let value = value.unwrap_or(AttributeValue::String(String::new()));

    // Determine attribute value type based on parsed components
    match object_type {
        Some("signal") => {
            if let (Some(msg_id), Some(sig_name)) = (message_id, signal_name) {
                let msg_id = if msg_id & (1 << 31) != 0 {
                    MessageId::Extended(msg_id & 0x1FFF_FFFF)
                } else {
                    MessageId::Standard(msg_id as u16)
                };
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Signal(msg_id, sig_name, value),
                })
            } else {
                todo!()
                // Ok(AttributeValueForObject {
                //     name: attribute_name,
                //     value: AttributeValuedForObjectType::Raw(value),
                // })
            }
        }
        Some("message") => {
            if let Some(msg_id) = message_id {
                let msg_id = if msg_id & (1 << 31) != 0 {
                    MessageId::Extended(msg_id & 0x1FFF_FFFF)
                } else {
                    MessageId::Standard(msg_id as u16)
                };
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::MessageDefinition(msg_id, Some(value)),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        Some("node") => {
            if let Some(node) = node_name {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::NetworkNode(node, value),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        Some("env_var") => {
            if let Some(env_var) = env_var_name {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::EnvVariable(env_var, value),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        _ => Ok(AttributeValueForObject {
            name: attribute_name,
            value: AttributeValuedForObjectType::Raw(value),
        }),
    }
}

/// Parse value table: VAL_TABLE_ table_name value1 "description1" value2 "description2" ... ;
pub(crate) fn parse_value_table(pair: Pair<Rule>) -> DbcResult<ValueTable> {
    let mut inner_pairs = pair.into_inner();

    let table_name = next_rule(&mut inner_pairs, Rule::table_name)?
        .as_str()
        .to_string();

    // Collect table value descriptions
    let mut descriptions = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::table_value_description {
            descriptions.push(parse_table_value_description(pair2)?);
        }
    }

    Ok(ValueTable {
        name: table_name,
        descriptions,
    })
}

/// Helper function to get the next pair and validate its rule
fn next_rule<'a>(iter: &'a mut Pairs<Rule>, expected_rule: Rule) -> DbcResult<Pair<'a, Rule>> {
    let pair = iter.next().ok_or(DbcError::ParseError)?;
    if pair.as_rule() != expected_rule {
        Err(DbcError::ParseError)
    } else {
        Ok(pair)
    }
}

#[allow(dead_code)]
fn peek_rule<'a>(iter: &mut Pairs<'a, Rule>, expected_rule: Rule) -> Option<Pair<'a, Rule>> {
    if let Some(pair) = iter.peek() {
        if pair.as_rule() == expected_rule {
            return Some(iter.next().unwrap());
        }
    }
    None
}

/// Helper function to ensure the iterator is empty (no more items)
#[allow(dead_code)]
fn expect_empty<'a>(iter: &mut Pairs<Rule>) -> DbcResult<()> {
    if iter.next().is_some() {
        Err(DbcError::ParseError)
    } else {
        Ok(())
    }
}

/// Helper to parse a single table value description pair (value + description)
pub(crate) fn parse_table_value_description(pair: Pair<Rule>) -> DbcResult<ValDescription> {
    let mut inner_pairs = pair.into_inner();

    let id = parse_int(next_rule(&mut inner_pairs, Rule::int)?)? as f64;
    let description = parse_str(next_rule(&mut inner_pairs, Rule::quoted_str)?);
    // Don't use expect_empty here as there might be comments or whitespace

    Ok(ValDescription { id, description })
}

/// Helper to parse min/max values from a min_max rule
pub(crate) fn parse_min_max_int(pair: Pair<Rule>) -> DbcResult<(i64, i64)> {
    let mut inner_pairs = pair.into_inner();

    let min_val = parse_int(next_rule(&mut inner_pairs, Rule::minimum)?)?;
    let max_val = parse_int(next_rule(&mut inner_pairs, Rule::maximum)?)?;
    // Don't use expect_empty here as there might be comments or whitespace

    Ok((min_val, max_val))
}

/// Helper to parse min/max values from a min_max rule as floats
pub(crate) fn parse_min_max_float(pair: Pair<Rule>) -> DbcResult<(f64, f64)> {
    let mut inner_pairs = pair.into_inner();

    let min_val = parse_float(next_rule(&mut inner_pairs, Rule::minimum)?)?;
    let max_val = parse_float(next_rule(&mut inner_pairs, Rule::maximum)?)?;
    // Don't use expect_empty here as there might be comments or whitespace

    Ok((min_val, max_val))
}

/// Parse value description: VAL_ message_id signal_name value1 "description1" value2 "description2" ... ;
pub(crate) fn parse_value_description(pair: Pair<Rule>) -> DbcResult<ValueDescription> {
    let mut inner_pairs = pair.into_inner();

    // Check if first item is message_id (optional)
    let mut message_id = None;
    if let Some(first_pair) = inner_pairs.next() {
        if first_pair.as_rule() == Rule::message_id {
            message_id = Some(parse_uint(first_pair)? as u32);
        } else {
            // Put it back and treat as signal_name (environment variable case)
            let signal_name = first_pair.as_str().to_string();
            let mut descriptions = Vec::new();
            for pair2 in inner_pairs {
                if pair2.as_rule() == Rule::table_value_description {
                    descriptions.push(parse_table_value_description(pair2)?);
                }
            }
            return Ok(ValueDescription::EnvironmentVariable {
                name: signal_name,
                value_descriptions: descriptions,
            });
        }
    }

    let signal_name = next_rule(&mut inner_pairs, Rule::signal_name)?
        .as_str()
        .to_string();

    // Collect table value descriptions
    let mut descriptions = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::table_value_description {
            descriptions.push(parse_table_value_description(pair2)?);
        }
    }

    if let Some(msg_id) = message_id {
        let msg_id = if msg_id & (1 << 31) != 0 {
            MessageId::Extended(msg_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(msg_id as u16)
        };
        Ok(ValueDescription::Signal {
            message_id: msg_id,
            name: signal_name,
            value_descriptions: descriptions,
        })
    } else {
        Ok(ValueDescription::EnvironmentVariable {
            name: signal_name,
            value_descriptions: descriptions,
        })
    }
}

/// Parse signal group: SIG_GROUP_ message_id group_name multiplexer_id : signal1 signal2 ... ;
pub(crate) fn parse_signal_group(pair: Pair<Rule>) -> DbcResult<SignalGroups> {
    let mut inner_pairs = pair.into_inner();

    let message_id = parse_uint(next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;
    let group_name = next_rule(&mut inner_pairs, Rule::group_name)?
        .as_str()
        .to_string();
    let repetitions = parse_uint(next_rule(&mut inner_pairs, Rule::multiplexer_id)?)?;

    // Collect remaining signal names
    let mut signal_names = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::signal_name {
            signal_names.push(pair2.as_str().to_string());
        }
    }

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    Ok(SignalGroups {
        message_id: msg_id,
        name: group_name,
        repetitions,
        signal_names,
    })
}

/// Parse signal value type: SIG_VALTYPE_ message_id signal_name : value_type;
pub(crate) fn parse_signal_value_type(pair: Pair<Rule>) -> DbcResult<SignalExtendedValueTypeList> {
    let mut inner_pairs = pair.into_inner();

    let message_id = parse_uint(next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;
    let signal_name = next_rule(&mut inner_pairs, Rule::signal_name)?
        .as_str()
        .to_string();
    let value_type = parse_uint(next_rule(&mut inner_pairs, Rule::int)?)?;

    // Don't use expect_empty here as there might be comments or whitespace

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    let signal_extended_value_type = match value_type {
        0 => SignalExtendedValueType::SignedOrUnsignedInteger,
        1 => SignalExtendedValueType::IEEEfloat32Bit,
        2 => SignalExtendedValueType::IEEEdouble64bit,
        _ => SignalExtendedValueType::SignedOrUnsignedInteger,
    };

    Ok(SignalExtendedValueTypeList {
        message_id: msg_id,
        signal_name,
        signal_extended_value_type,
    })
}

/// Parse message transmitter: BO_TX_BU_ message_id : transmitter1,transmitter2,... ;
pub(crate) fn parse_message_transmitter(pair: Pair<Rule>) -> DbcResult<MessageTransmitter> {
    let mut inner_pairs = pair.into_inner();

    let message_id = parse_uint(next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;

    // Collect transmitters
    let mut transmitters = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::transmitter {
            let name = pair2.as_str().to_string();
            let transmitter = if name == "Vector__XXX" {
                Transmitter::VectorXXX
            } else {
                Transmitter::NodeName(name)
            };
            transmitters.push(transmitter);
        }
    }

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    Ok(MessageTransmitter {
        message_id: msg_id,
        transmitter: transmitters,
    })
}

/// Parse attribute default: BA_DEF_DEF_ attribute_name default_value;
pub(crate) fn parse_attribute_default(pair: Pair<Rule>) -> DbcResult<AttributeDefault> {
    let mut inner_pairs = pair.into_inner();

    let attribute_name = parse_str(next_rule(&mut inner_pairs, Rule::attribute_name)?);

    // Parse the value - could be quoted_str or number (num_str_value is silent)
    let value_pair = inner_pairs.next().ok_or(DbcError::ParseError)?;
    let default_value = match value_pair.as_rule() {
        Rule::quoted_str => AttributeValue::String(parse_str(value_pair)),
        Rule::number => AttributeValue::Double(parse_float(value_pair)?),
        _ => return Err(DbcError::ParseError),
    };

    // Don't use expect_empty here as there might be comments or whitespace

    Ok(AttributeDefault {
        name: attribute_name,
        value: default_value,
    })
}

/// Parse extended multiplex: SG_MUL_VAL_ message_id signal_name multiplexor_name value_pairs;
pub(crate) fn parse_extended_multiplex(pair: Pair<Rule>) -> DbcResult<ExtendedMultiplex> {
    let mut inner_pairs = pair.into_inner();

    let message_id = parse_uint(next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;
    let signal_name = next_rule(&mut inner_pairs, Rule::signal_name)?
        .as_str()
        .to_string();
    let multiplexor_name = next_rule(&mut inner_pairs, Rule::multiplexer_name)?
        .as_str()
        .to_string();

    // Collect value pairs
    let mut mappings = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::value_pair {
            let mut min_val = None;
            let mut max_val = None;
            for pair3 in pair2.into_inner() {
                if pair3.as_rule() == Rule::int {
                    let value = parse_uint(pair3)?;
                    if min_val.is_none() {
                        min_val = Some(value);
                    } else {
                        max_val = Some(value);
                    }
                }
            }
            if let (Some(min), Some(max)) = (min_val, max_val) {
                mappings.push(ExtendedMultiplexMapping {
                    min_value: min,
                    max_value: max,
                });
            }
        }
    }

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    Ok(ExtendedMultiplex {
        message_id: msg_id,
        signal_name,
        multiplexor_signal_name: multiplexor_name,
        mappings,
    })
}

/// Parse environment variable: EV_ variable_name : type [min|max] "unit" access_type access_node node_name1 node_name2;
pub(crate) fn parse_environment_variable(pair: Pair<Rule>) -> DbcResult<EnvironmentVariable> {
    let mut variable_name = String::new();
    let mut env_type = 0u64;
    let mut min_val = 0i64;
    let mut max_val = 0i64;
    let mut unit = String::new();
    let mut initial_value = 0.0f64;
    let mut ev_id = 0i64;
    let mut access_type = String::new();
    let mut access_nodes = Vec::new();

    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::env_var => {
                // env_var contains env_literal ~ env_var_name
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::env_var_name {
                        variable_name = sub_pair.as_str().to_string();
                    }
                }
            }
            Rule::env_var_type_int => env_type = 0, // Integer type
            Rule::env_var_type_float => env_type = 1, // Float type
            Rule::env_var_type_string => env_type = 2, // String type
            Rule::min_max => (min_val, max_val) = parse_min_max_int(pair2)?,
            Rule::unit => unit = parse_str(pair2),
            Rule::init_value => initial_value = parse_int(pair2)? as f64,
            Rule::ev_id => ev_id = parse_int(pair2)?,
            Rule::node_name => {
                let name = pair2.as_str().to_string();
                if access_type.is_empty() {
                    // First node_name is the access type
                    access_type = name;
                } else {
                    // Subsequent node_names are access nodes
                    let access_node = if name == "VECTOR__XXX" {
                        AccessNode::VectorXXX
                    } else {
                        AccessNode::Name(name)
                    };
                    access_nodes.push(access_node);
                }
            }
            other => panic!("What is this? {other:?}"),
        }
    }

    let typ = match env_type {
        0 => EnvType::Float,
        1 => EnvType::U64,
        2 => EnvType::Data,
        _ => EnvType::Float,
    };

    let access_type_enum = match access_type.as_str() {
        "DUMMY_NODE_VECTOR0" => AccessType::DummyNodeVector0,
        "DUMMY_NODE_VECTOR1" => AccessType::DummyNodeVector1,
        "DUMMY_NODE_VECTOR2" => AccessType::DummyNodeVector2,
        "DUMMY_NODE_VECTOR3" => AccessType::DummyNodeVector3,
        _ => AccessType::DummyNodeVector0,
    };

    Ok(EnvironmentVariable {
        name: variable_name,
        typ,
        min: min_val,
        max: max_val,
        unit,
        initial_value,
        ev_id,
        access_type: access_type_enum,
        access_nodes,
    })
}

/// Parse signal: SG_ signal_name : start_bit|signal_size@byte_order+/- (factor,offset) [min|max] "unit" receiver
pub(crate) fn parse_signal(pair: Pair<Rule>) -> DbcResult<Signal> {
    let mut signal_name = String::new();
    let mut multiplexer_indicator = MultiplexIndicator::Plain;
    let mut start_bit = 0u64;
    let mut signal_size = 0u64;
    let mut byte_order = ByteOrder::BigEndian;
    let mut value_type = ValueType::Unsigned;
    let mut factor = 0.0f64;
    let mut offset = 0.0f64;
    let mut min = 0.0f64;
    let mut max = 0.0f64;
    let mut unit = String::new();
    let mut receivers = Vec::new();

    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::signal_name => {
                signal_name = pair2.as_str().to_string();
            }
            Rule::multiplexer_indicator => {
                let text = pair2.as_str();
                if text == "M" {
                    multiplexer_indicator = MultiplexIndicator::Multiplexor;
                } else if text.starts_with('m') {
                    // Parse multiplexed signal value from the text
                    // The text should be like "m1" or "m1M"
                    if text.len() > 1 {
                        let value_str = &text[1..];
                        // Check if it ends with 'M' (multiplexor and multiplexed signal)
                        if value_str.ends_with('M') {
                            let value_str = &value_str[..value_str.len() - 1];
                            if let Ok(value) = value_str.parse::<u64>() {
                                multiplexer_indicator =
                                    MultiplexIndicator::MultiplexorAndMultiplexedSignal(value);
                            }
                        } else {
                            // Just multiplexed signal
                            if let Ok(value) = value_str.parse::<u64>() {
                                multiplexer_indicator =
                                    MultiplexIndicator::MultiplexedSignal(value);
                            }
                        }
                    }
                }
            }
            Rule::start_bit => start_bit = parse_uint(pair2)?,
            Rule::signal_size => signal_size = parse_uint(pair2)?,
            Rule::big_endian => byte_order = ByteOrder::BigEndian,
            Rule::little_endian => byte_order = ByteOrder::LittleEndian,
            Rule::signed_type => value_type = ValueType::Signed,
            Rule::unsigned_type => value_type = ValueType::Unsigned,
            Rule::factor => factor = parse_float(pair2)?,
            Rule::offset => offset = parse_float(pair2)?,
            Rule::min_max => (min, max) = parse_min_max_float(pair2)?,
            Rule::unit => unit = parse_str(pair2),
            Rule::node_name => receivers.push(pair2.as_str().to_string()),
            other => panic!("What is this? {other:?}"),
        }
    }

    Ok(Signal {
        name: signal_name,
        multiplexer_indicator,
        start_bit,
        size: signal_size,
        byte_order,
        value_type,
        factor,
        offset,
        min,
        max,
        unit,
        receivers,
    })
}

pub(crate) fn parse_environment_variable_data(
    pair: Pair<Rule>,
) -> DbcResult<EnvironmentVariableData> {
    let mut inner_pairs = pair.into_inner();

    let variable_name = next_rule(&mut inner_pairs, Rule::env_var_name)?
        .as_str()
        .to_string();
    let data_size = parse_uint(next_rule(&mut inner_pairs, Rule::data_size)?)?;

    // Don't use expect_empty here as there might be comments or whitespace

    Ok(EnvironmentVariableData {
        env_var_name: variable_name,
        data_size,
    })
}

pub(crate) fn dbc(buffer: &str) -> DbcResult<Dbc> {
    let pairs = DbcParser::parse(Rule::file, buffer)?;

    let mut version: Version = Default::default();
    let mut new_symbols: Vec<Symbol> = Default::default();
    let mut bit_timing: Option<Vec<Baudrate>> = Default::default();
    let mut nodes: Vec<Node> = Default::default();
    let mut value_tables: Vec<ValueTable> = Default::default();
    let mut messages: Vec<Message> = Default::default();
    let mut signals: Vec<(MessageId, Signal)> = Default::default(); // Store signals with their message ID
    let mut message_transmitters: Vec<MessageTransmitter> = Default::default();
    let mut environment_variables: Vec<EnvironmentVariable> = Default::default();
    let mut environment_variable_data: Vec<EnvironmentVariableData> = Default::default();
    let signal_types: Vec<SignalType> = Default::default();
    let mut comments: Vec<Comment> = Default::default();
    let mut attribute_definitions: Vec<AttributeDefinition> = Default::default();
    let mut attribute_defaults: Vec<AttributeDefault> = Default::default();
    let mut attribute_values: Vec<AttributeValueForObject> = Default::default();
    let mut value_descriptions: Vec<ValueDescription> = Default::default();
    let signal_type_refs: Vec<SignalTypeRef> = Default::default();
    let mut signal_groups: Vec<SignalGroups> = Default::default();
    let mut signal_extended_value_type_list: Vec<SignalExtendedValueTypeList> = Default::default();
    let mut extended_multiplex: Vec<ExtendedMultiplex> = Default::default();

    let mut current_message_id: Option<MessageId> = None;

    for pair in pairs {
        if !matches!(pair.as_rule(), Rule::file) {
            return Err(DbcError::ParseError);
        }
        for pair2 in pair.into_inner() {
            match pair2.as_rule() {
                Rule::version => version = parse_version(pair2)?,
                Rule::new_symbols => new_symbols = parse_new_symbols(pair2)?,
                Rule::bit_timing => bit_timing = Some(parse_bit_timing(pair2)?),
                Rule::nodes => nodes = parse_nodes(pair2)?,
                Rule::message => {
                    let message = parse_message(pair2)?;
                    current_message_id = Some(message.id);
                    messages.push(message);
                }
                Rule::signal => {
                    if let Some(msg_id) = current_message_id {
                        signals.push((msg_id, parse_signal(pair2)?));
                    }
                }
                Rule::comment => {
                    if let Some(comment) = parse_comment(pair2)? {
                        comments.push(comment);
                    }
                }
                Rule::attr_def => attribute_definitions.push(parse_attribute_definition(pair2)?),
                Rule::attr_value => attribute_values.push(parse_attribute_value(pair2)?),
                Rule::value_table => value_tables.push(parse_value_table(pair2)?),
                Rule::value_table_def => value_descriptions.push(parse_value_description(pair2)?),
                Rule::signal_group => signal_groups.push(parse_signal_group(pair2)?),
                Rule::signal_value_type => {
                    signal_extended_value_type_list.push(parse_signal_value_type(pair2)?)
                }
                Rule::bo_tx_bu => message_transmitters.push(parse_message_transmitter(pair2)?),
                Rule::ba_def_def => attribute_defaults.push(parse_attribute_default(pair2)?),
                Rule::sg_mul_val => extended_multiplex.push(parse_extended_multiplex(pair2)?),
                Rule::environment_variable => {
                    environment_variables.push(parse_environment_variable(pair2)?)
                }
                Rule::envvar_data => {
                    environment_variable_data.push(parse_environment_variable_data(pair2)?)
                }
                Rule::EOI => {}
                other => panic!("What is this? {other:?}"),
            }
        }
    }

    // Associate signals with their messages
    for (msg_id, signal) in signals {
        if let Some(message) = messages.iter_mut().find(|m| m.id == msg_id) {
            message.signals.push(signal);
        }
    }

    Ok(Dbc {
        version,
        new_symbols,
        bit_timing,
        nodes,
        value_tables,
        messages,
        message_transmitters,
        environment_variables,
        environment_variable_data,
        signal_types,
        comments,
        attribute_definitions,
        attribute_defaults,
        attribute_values,
        value_descriptions,
        signal_type_refs,
        signal_groups,
        signal_extended_value_type_list,
        extended_multiplex,
    })
}
