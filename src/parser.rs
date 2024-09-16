//!
//! Module containing nom parser combinators
//!

use std::str;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till, take_while, take_while1},
    character::complete::{self, char, line_ending, multispace0, space0, space1},
    combinator::{map, opt, value},
    error::{ErrorKind, ParseError},
    multi::{many0, many_till, separated_list0},
    number::complete::double,
    sequence::preceded,
    AsChar, IResult, InputTakeAtPosition,
};

use crate::message::{Message, MessageId};
use crate::signal::{Signal, SignalType, MultiplexIndicator, ByteOrder, ValueType, ExtendedMultiplexMapping, ExtendedMultiplex, SignalGroups, SignalExtendedValueType, SignalExtendedValueTypeList, SignalTypeRef};
use crate::nodes::{Node, AccessNode, AccessType, Transmitter, MessageTransmitter};
use crate::dbc::{Version, Baudrate, Symbol, DBC, Comment};
use crate::attributes::{AttributeDefault, AttributeValueForObject, AttributeDefinition, AttributeValue, ValueDescription, AttributeValuedForObjectType, ValDescription, ValueTable};
use crate::env_variables::{EnvironmentVariable, EnvType, EnvironmentVariableData};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_ident_test() {
        let def1 = "EALL_DUSasb18 ";
        let (_, cid1) = c_ident(def1).unwrap();
        assert_eq!("EALL_DUSasb18", cid1);

        let def2 = "_EALL_DUSasb18 ";
        let (_, cid2) = c_ident(def2).unwrap();
        assert_eq!("_EALL_DUSasb18", cid2);

        // identifiers must not start with digit1s
        let def3 = "3EALL_DUSasb18 ";
        let cid3_result = c_ident(def3);
        assert!(cid3_result.is_err());
    }

    #[test]
    fn c_ident_vec_test() {
        let cid = "FZHL_DUSasb18 ";
        let (_, cid1) = c_ident_vec(cid).unwrap();

        assert_eq!(vec!("FZHL_DUSasb18".to_string()), cid1);

        let cid_vec = "FZHL_DUSasb19,xkask_3298 ";
        let (_, cid2) = c_ident_vec(cid_vec).unwrap();

        assert_eq!(
            vec!("FZHL_DUSasb19".to_string(), "xkask_3298".to_string()),
            cid2
        );
    }

    #[test]
    fn char_string_test() {
        let def = "\"ab\x00\x7f\"";
        let (_, char_string) = char_string(def).unwrap();
        let exp = "ab\x00\x7f";
        assert_eq!(exp, char_string);
    }

    #[test]
    fn signal_test() {
        let signal_line = "SG_ NAME : 3|2@1- (1,0) [0|0] \"x\" UFA\r\n";
        let _signal = signal(signal_line).unwrap();
    }

    #[test]
    fn byte_order_test() {
        let (_, big_endian) = byte_order("0").expect("Failed to parse big endian");
        assert_eq!(ByteOrder::BigEndian, big_endian);

        let (_, little_endian) = byte_order("1").expect("Failed to parse little endian");
        assert_eq!(ByteOrder::LittleEndian, little_endian);
    }

    #[test]
    fn multiplexer_indicator_test() {
        let (_, multiplexer) =
            multiplexer_indicator(" m34920 eol").expect("Failed to parse multiplexer");
        assert_eq!(MultiplexIndicator::MultiplexedSignal(34920), multiplexer);

        let (_, multiplexor) =
            multiplexer_indicator(" M eol").expect("Failed to parse multiplexor");
        assert_eq!(MultiplexIndicator::Multiplexor, multiplexor);

        let (_, plain) = multiplexer_indicator(" eol").expect("Failed to parse plain");
        assert_eq!(MultiplexIndicator::Plain, plain);

        let (_, multiplexer) =
            multiplexer_indicator(" m8M eol").expect("Failed to parse multiplexer");
        assert_eq!(
            MultiplexIndicator::MultiplexorAndMultiplexedSignal(8),
            multiplexer
        );
    }

    #[test]
    fn value_type_test() {
        let (_, vt) = value_type("- ").expect("Failed to parse value type");
        assert_eq!(ValueType::Signed, vt);

        let (_, vt) = value_type("+ ").expect("Failed to parse value type");
        assert_eq!(ValueType::Unsigned, vt);
    }

    #[test]
    fn message_definition_test() {
        let def = "BO_ 1 MCA_A1: 6 MFA\r\nSG_ ABC_1 : 9|2@1+ (1,0) [0|0] \"x\" XYZ_OUS\r\nSG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n x";
        signal("\r\n\r\nSG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n").expect("Failed");
        let (_, _message_def) = message(def).expect("Failed to parse message definition");
    }

    #[test]
    fn signal_comment_test() {
        let def1 = "CM_ SG_ 193 KLU_R_X \"This is a signal comment test\";\n";
        let message_id = MessageId::Standard(193);
        let comment1 = Comment::Signal {
            message_id,
            signal_name: "KLU_R_X".to_string(),
            comment: "This is a signal comment test".to_string(),
        };
        let (_, comment1_def) = comment(def1).expect("Failed to parse signal comment definition");
        assert_eq!(comment1, comment1_def);
    }

    #[test]
    fn message_definition_comment_test() {
        let def1 = "CM_ BO_ 34544 \"Some Message comment\";\n";
        let message_id = MessageId::Standard(34544);
        let comment1 = Comment::Message {
            message_id,
            comment: "Some Message comment".to_string(),
        };
        let (_, comment1_def) =
            comment(def1).expect("Failed to parse message definition comment definition");
        assert_eq!(comment1, comment1_def);
    }

    #[test]
    fn node_comment_test() {
        let def1 = "CM_ BU_ network_node \"Some network node comment\";\n";
        let comment1 = Comment::Node {
            node_name: "network_node".to_string(),
            comment: "Some network node comment".to_string(),
        };
        let (_, comment1_def) = comment(def1).expect("Failed to parse node comment definition");
        assert_eq!(comment1, comment1_def);
    }

    #[test]
    fn env_var_comment_test() {
        let def1 = "CM_ EV_ ENVXYZ \"Some env var name comment\";\n";
        let comment1 = Comment::EnvVar {
            env_var_name: "ENVXYZ".to_string(),
            comment: "Some env var name comment".to_string(),
        };
        let (_, comment1_def) = comment(def1).expect("Failed to parse env var comment definition");
        assert_eq!(comment1, comment1_def);
    }

    #[test]
    fn value_description_for_signal_test() {
        let def1 = "VAL_ 837 UF_HZ_OI 255 \"NOP\";\n";
        let message_id = MessageId::Standard(837);
        let signal_name = "UF_HZ_OI".to_string();
        let val_descriptions = vec![ValDescription {
            a: 255.0,
            b: "NOP".to_string(),
        }];
        let value_description_for_signal1 = ValueDescription::Signal {
            message_id,
            signal_name,
            value_descriptions: val_descriptions,
        };
        let (_, value_signal_def) =
            value_descriptions(def1).expect("Failed to parse value desc for signal");
        assert_eq!(value_description_for_signal1, value_signal_def);
    }

    #[test]
    fn value_description_for_env_var_test() {
        let def1 = "VAL_ MY_ENV_VAR 255 \"NOP\";\n";
        let env_var_name = "MY_ENV_VAR".to_string();
        let val_descriptions = vec![ValDescription {
            a: 255.0,
            b: "NOP".to_string(),
        }];
        let value_env_var1 = ValueDescription::EnvironmentVariable {
            env_var_name,
            value_descriptions: val_descriptions,
        };
        let (_, value_env_var) =
            value_descriptions(def1).expect("Failed to parse value desc for env var");
        assert_eq!(value_env_var1, value_env_var);
    }

    #[test]
    fn environment_variable_test() {
        let def1 = "EV_ IUV: 0 [-22|20] \"mm\" 3 7 DUMMY_NODE_VECTOR0 VECTOR_XXX;\n";
        let env_var1 = EnvironmentVariable {
            env_var_name: "IUV".to_string(),
            env_var_type: EnvType::EnvTypeFloat,
            min: -22,
            max: 20,
            unit: "mm".to_string(),
            initial_value: 3.0,
            ev_id: 7,
            access_type: AccessType::DummyNodeVector0,
            access_nodes: vec![AccessNode::AccessNodeVectorXXX],
        };
        let (_, env_var) =
            environment_variable(def1).expect("Failed to parse environment variable");
        assert_eq!(env_var1, env_var);
    }

    #[test]
    fn network_node_attribute_value_test() {
        let def = "BA_ \"AttrName\" BU_ NodeName 12;\n";
        let attribute_value = AttributeValuedForObjectType::NetworkNodeAttributeValue(
            "NodeName".to_string(),
            AttributeValue::AttributeValueF64(12.0),
        );
        let attr_val_exp = AttributeValueForObject {
            attribute_name: "AttrName".to_string(),
            attribute_value,
        };
        let (_, attr_val) = attribute_value_for_object(def).unwrap();
        assert_eq!(attr_val_exp, attr_val);
    }

    #[test]
    fn message_definition_attribute_value_test() {
        let def = "BA_ \"AttrName\" BO_ 298 13;\n";
        let attribute_value = AttributeValuedForObjectType::MessageDefinitionAttributeValue(
            MessageId::Standard(298),
            Some(AttributeValue::AttributeValueF64(13.0)),
        );
        let attr_val_exp = AttributeValueForObject {
            attribute_name: "AttrName".to_string(),
            attribute_value,
        };
        let (_, attr_val) = attribute_value_for_object(def).unwrap();
        assert_eq!(attr_val_exp, attr_val);
    }

    #[test]
    fn signal_attribute_value_test() {
        let def = "BA_ \"AttrName\" SG_ 198 SGName 13;\n";
        let attribute_value = AttributeValuedForObjectType::SignalAttributeValue(
            MessageId::Standard(198),
            "SGName".to_string(),
            AttributeValue::AttributeValueF64(13.0),
        );
        let attr_val_exp = AttributeValueForObject {
            attribute_name: "AttrName".to_string(),
            attribute_value,
        };
        let (_, attr_val) = attribute_value_for_object(def).unwrap();
        assert_eq!(attr_val_exp, attr_val);
    }

    #[test]
    fn env_var_attribute_value_test() {
        let def = "BA_ \"AttrName\" EV_ EvName \"CharStr\";\n";
        let attribute_value = AttributeValuedForObjectType::EnvVariableAttributeValue(
            "EvName".to_string(),
            AttributeValue::AttributeValueCharString("CharStr".to_string()),
        );
        let attr_val_exp = AttributeValueForObject {
            attribute_name: "AttrName".to_string(),
            attribute_value,
        };
        let (_, attr_val) = attribute_value_for_object(def).unwrap();
        assert_eq!(attr_val_exp, attr_val);
    }

    #[test]
    fn raw_attribute_value_test() {
        let def = "BA_ \"AttrName\" \"RAW\";\n";
        let attribute_value = AttributeValuedForObjectType::RawAttributeValue(
            AttributeValue::AttributeValueCharString("RAW".to_string()),
        );
        let attr_val_exp = AttributeValueForObject {
            attribute_name: "AttrName".to_string(),
            attribute_value,
        };
        let (_, attr_val) = attribute_value_for_object(def).unwrap();
        assert_eq!(attr_val_exp, attr_val);
    }

    #[test]
    fn new_symbols_test() {
        let def = "NS_ :
                NS_DESC_
                CM_
                BA_DEF_

            ";
        let symbols_exp = vec![
            Symbol("NS_DESC_".to_string()),
            Symbol("CM_".to_string()),
            Symbol("BA_DEF_".to_string()),
        ];
        let (_, symbols) = new_symbols(def).unwrap();
        assert_eq!(symbols_exp, symbols);
    }

    #[test]
    fn network_node_test() {
        let def = "BU_: ZU XYZ ABC OIU\n";
        let nodes = vec![
            "ZU".to_string(),
            "XYZ".to_string(),
            "ABC".to_string(),
            "OIU".to_string(),
        ];
        let (_, node) = node(def).unwrap();
        let node_exp = Node(nodes);
        assert_eq!(node_exp, node);
    }

    #[test]
    fn empty_network_node_test() {
        let def = "BU_: \n";
        let nodes = vec![];
        let (_, node) = node(def).unwrap();
        let node_exp = Node(nodes);
        assert_eq!(node_exp, node);
    }

    #[test]
    fn envvar_data_test() {
        let def = "ENVVAR_DATA_ SomeEnvVarData: 399;\n";
        let (_, envvar_data) = environment_variable_data(def).unwrap();
        let envvar_data_exp = EnvironmentVariableData {
            env_var_name: "SomeEnvVarData".to_string(),
            data_size: 399,
        };
        assert_eq!(envvar_data_exp, envvar_data);
    }

    #[test]
    fn signal_type_test() {
        let def = "SGTYPE_ signal_type_name: 1024@1+ (5,2) [1|3] \"unit\" 2.0 val_table;\n";

        let exp = SignalType {
            signal_type_name: "signal_type_name".to_string(),
            signal_size: 1024,
            byte_order: ByteOrder::LittleEndian,
            value_type: ValueType::Unsigned,
            factor: 5.0,
            offset: 2.0,
            min: 1.0,
            max: 3.0,
            unit: "unit".to_string(),
            default_value: 2.0,
            value_table: "val_table".to_string(),
        };

        let (_, signal_type) = signal_type(def).unwrap();
        assert_eq!(exp, signal_type);
    }

    #[test]
    fn signal_groups_test() {
        let def = "SIG_GROUP_ 23 X_3290 1 : A_b XY_Z;\n";

        let exp = SignalGroups {
            message_id: MessageId::Standard(23),
            signal_group_name: "X_3290".to_string(),
            repetitions: 1,
            signal_names: vec!["A_b".to_string(), "XY_Z".to_string()],
        };

        let (_, signal_groups) = signal_groups(def).unwrap();
        assert_eq!(exp, signal_groups);
    }

    #[test]
    fn attribute_default_test() {
        let def = "BA_DEF_DEF_  \"ZUV\" \"OAL\";\n";
        let (_, attr_default) = attribute_default(def).unwrap();
        let attr_default_exp = AttributeDefault {
            attribute_name: "ZUV".to_string(),
            attribute_value: AttributeValue::AttributeValueCharString("OAL".to_string()),
        };
        assert_eq!(attr_default_exp, attr_default);
    }

    #[test]
    fn attribute_value_f64_test() {
        let def = "80.0";
        let (_, val) = attribute_value(def).unwrap();
        assert_eq!(AttributeValue::AttributeValueF64(80.0), val);
    }

    #[test]
    fn attribute_definition_test() {
        let def_bo = "BA_DEF_ BO_ \"BaDef1BO\" INT 0 1000000;\n";
        let (_, bo_def) = attribute_definition(def_bo).unwrap();
        let bo_def_exp = AttributeDefinition::Message("\"BaDef1BO\" INT 0 1000000".to_string());
        assert_eq!(bo_def_exp, bo_def);

        let def_bu = "BA_DEF_ BU_ \"BuDef1BO\" INT 0 1000000;\n";
        let (_, bu_def) = attribute_definition(def_bu).unwrap();
        let bu_def_exp = AttributeDefinition::Node("\"BuDef1BO\" INT 0 1000000".to_string());
        assert_eq!(bu_def_exp, bu_def);

        let def_signal = "BA_DEF_ SG_ \"SgDef1BO\" INT 0 1000000;\n";
        let (_, signal_def) = attribute_definition(def_signal).unwrap();
        let signal_def_exp = AttributeDefinition::Signal("\"SgDef1BO\" INT 0 1000000".to_string());
        assert_eq!(signal_def_exp, signal_def);

        let def_env_var = "BA_DEF_ EV_ \"EvDef1BO\" INT 0 1000000;\n";
        let (_, env_var_def) = attribute_definition(def_env_var).unwrap();
        let env_var_def_exp =
            AttributeDefinition::EnvironmentVariable("\"EvDef1BO\" INT 0 1000000".to_string());
        assert_eq!(env_var_def_exp, env_var_def);
    }

    #[test]
    fn version_test() {
        let def = "VERSION \"HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///\"\n";
        let version_exp =
            Version("HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///".to_string());
        let (_, version) = version(def).unwrap();
        assert_eq!(version_exp, version);
    }

    #[test]
    fn message_transmitters_test() {
        let def = "BO_TX_BU_ 12345 : XZY,ABC;\n";
        let exp = MessageTransmitter {
            message_id: MessageId::Standard(12345),
            transmitter: vec![
                Transmitter::NodeName("XZY".to_string()),
                Transmitter::NodeName("ABC".to_string()),
            ],
        };
        let (_, transmitter) = message_transmitter(def).unwrap();
        assert_eq!(exp, transmitter);
    }

    #[test]
    fn value_description_test() {
        let def = "2 \"ABC\"\n";
        let exp = ValDescription {
            a: 2f64,
            b: "ABC".to_string(),
        };
        let (_, val_desc) = value_description(def).unwrap();
        assert_eq!(exp, val_desc);
    }

    #[test]
    fn val_table_test() {
        let def = "VAL_TABLE_ Tst 2 \"ABC\" 1 \"Test A\" ;\n";
        let exp = ValueTable {
            value_table_name: "Tst".to_string(),
            value_descriptions: vec![
                ValDescription {
                    a: 2f64,
                    b: "ABC".to_string(),
                },
                ValDescription {
                    a: 1f64,
                    b: "Test A".to_string(),
                },
            ],
        };
        let (_, val_table) = value_table(def).unwrap();
        assert_eq!(exp, val_table);
    }

    #[test]
    fn val_table_no_space_preceding_comma_test() {
        let def = "VAL_TABLE_ Tst 2 \"ABC\";\n";
        let exp = ValueTable {
            value_table_name: "Tst".to_string(),
            value_descriptions: vec![ValDescription {
                a: 2f64,
                b: "ABC".to_string(),
            }],
        };
        let (_, val_table) = value_table(def).unwrap();
        assert_eq!(exp, val_table);
    }

    #[test]
    fn extended_multiplex_test() {
        // simple mapping
        let def = "SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1-1;\n";
        let exp = ExtendedMultiplex {
            message_id: MessageId::Extended(2),
            signal_name: "muxed_A_1".to_string(),
            multiplexor_signal_name: "MUX_A".to_string(),
            mappings: vec![ExtendedMultiplexMapping {
                min_value: 1,
                max_value: 1,
            }],
        };
        let (_, ext_multiplex) = extended_multiplex(def).unwrap();
        assert_eq!(exp, ext_multiplex);

        // range mapping
        let def = "SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1568-2568;\n";
        let exp = ExtendedMultiplex {
            message_id: MessageId::Extended(2),
            signal_name: "muxed_A_1".to_string(),
            multiplexor_signal_name: "MUX_A".to_string(),
            mappings: vec![ExtendedMultiplexMapping {
                min_value: 1568,
                max_value: 2568,
            }],
        };
        let (_, ext_multiplex) = extended_multiplex(def).unwrap();
        assert_eq!(exp, ext_multiplex);

        // multiple mappings
        let def = "SG_MUL_VAL_ 2147483650 muxed_B_5 MUX_B 5-5, 16-24;\n";
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
        let (_, ext_multiplex) = extended_multiplex(def).unwrap();
        assert_eq!(exp, ext_multiplex);
    }

    #[test]
    fn sig_val_type_test() {
        let def = "SIG_VALTYPE_ 2000 Signal_8 : 1;\n";
        let exp = SignalExtendedValueTypeList {
            message_id: MessageId::Standard(2000),
            signal_name: "Signal_8".to_string(),
            signal_extended_value_type: SignalExtendedValueType::IEEEfloat32Bit,
        };

        let (_, extended_value_type_list) = signal_extended_value_type_list(def).unwrap();
        assert_eq!(extended_value_type_list, exp);
    }

    #[test]
    fn standard_message_id_test() {
        let (_, extended_message_id) = message_id("2").unwrap();
        assert_eq!(extended_message_id, MessageId::Standard(2));
    }

    #[test]
    fn extended_low_message_id_test() {
        let s = (2u32 | 1 << 31).to_string();
        let (_, extended_message_id) = message_id(&s).unwrap();
        assert_eq!(extended_message_id, MessageId::Extended(2));
    }

    #[test]
    fn extended_message_id_test() {
        let s = (0x1FFFFFFFu32 | 1 << 31).to_string();
        let (_, extended_message_id) = message_id(&s).unwrap();
        assert_eq!(extended_message_id, MessageId::Extended(0x1FFFFFFF));
    }

    #[test]
    fn extended_message_id_test_max_29bit() {
        let s = u32::MAX.to_string();
        let (_, extended_message_id) = message_id(&s).unwrap();
        assert_eq!(extended_message_id, MessageId::Extended(0x1FFFFFFF));
    }
}

pub (crate) fn is_semi_colon(chr: char) -> bool {
    chr == ';'
}

pub (crate) fn is_c_string_char(chr: char) -> bool {
    chr.is_ascii_digit() || chr.is_alphabetic() || chr == '_'
}

pub (crate) fn is_c_ident_head(chr: char) -> bool {
    chr.is_alphabetic() || chr == '_'
}

pub (crate) fn is_quote(chr: char) -> bool {
    chr == '"'
}

/// Multispace zero or more
pub (crate) fn ms0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        c != ' '
    })
}

/// Multi space one or more
pub (crate) fn ms1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            c != ' '
        },
        ErrorKind::MultiSpace,
    )
}

/// Colon aka `:`
pub (crate) fn colon(s: &str) -> IResult<&str, char> {
    char(':')(s)
}

/// Comma aka ','
pub (crate) fn comma(s: &str) -> IResult<&str, char> {
    char(',')(s)
}

/// Comma aka ';'
pub (crate) fn semi_colon(s: &str) -> IResult<&str, char> {
    char(';')(s)
}

/// Quote aka '"'
pub (crate) fn quote(s: &str) -> IResult<&str, char> {
    char('"')(s)
}

/// Pipe character
pub (crate) fn pipe(s: &str) -> IResult<&str, char> {
    char('|')(s)
}

/// at character
pub (crate) fn at(s: &str) -> IResult<&str, char> {
    char('@')(s)
}

/// brace open aka '('
pub (crate) fn brc_open(s: &str) -> IResult<&str, char> {
    char('(')(s)
}

/// brace close aka ')'
pub (crate) fn brc_close(s: &str) -> IResult<&str, char> {
    char(')')(s)
}

/// bracket open aka '['
pub (crate) fn brk_open(s: &str) -> IResult<&str, char> {
    char('[')(s)
}

/// bracket close aka ']'
pub (crate) fn brk_close(s: &str) -> IResult<&str, char> {
    char(']')(s)
}

/// A valid C_identifier. C_identifiers start with a  alphacharacter or an underscore
/// and may further consist of alpha­numeric, characters and underscore
pub (crate) fn c_ident(s: &str) -> IResult<&str, String> {
    let (s, head) = take_while1(is_c_ident_head)(s)?;
    let (s, remaining) = take_while(is_c_string_char)(s)?;
    Ok((s, [head, remaining].concat()))
}

pub (crate) fn c_ident_vec(s: &str) -> IResult<&str, Vec<String>> {
    separated_list0(comma, c_ident)(s)
}

pub (crate) fn char_string(s: &str) -> IResult<&str, &str> {
    let (s, _) = quote(s)?;
    let (s, char_string_value) = take_till(is_quote)(s)?;
    let (s, _) = quote(s)?;
    Ok((s, char_string_value))
}

fn bit_timing(s: &str) -> IResult<&str, Vec<Baudrate>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BS_:")(s)?;
    let (s, baudrates) = opt(preceded(
        ms1,
        separated_list0(comma, map(complete::u64, Baudrate)),
    ))(s)?;
    Ok((s, baudrates.unwrap_or_default()))
}

fn signal(s: &str) -> IResult<&str, Signal> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SG_")(s)?;
    let (s, _) = ms1(s)?;
    let (s, name) = c_ident(s)?;
    let (s, multiplexer_indicator) = multiplexer_indicator(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, start_bit) = complete::u64(s)?;
    let (s, _) = pipe(s)?;
    let (s, signal_size) = complete::u64(s)?;
    let (s, _) = at(s)?;
    let (s, byte_order) = byte_order(s)?;
    let (s, value_type) = value_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, factor) = double(s)?;
    let (s, _) = comma(s)?;
    let (s, offset) = double(s)?;
    let (s, _) = brc_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brk_open(s)?;
    let (s, min) = double(s)?;
    let (s, _) = pipe(s)?;
    let (s, max) = double(s)?;
    let (s, _) = brk_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, unit) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, receivers) = c_ident_vec(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        Signal {
            name,
            multiplexer_indicator,
            start_bit,
            signal_size,
            byte_order,
            value_type,
            factor,
            offset,
            min,
            max,
            unit: unit.to_string(),
            receivers,
        },
    ))
}

fn attribute_default(s: &str) -> IResult<&str, AttributeDefault> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BA_DEF_DEF_")(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_name) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_value) = attribute_value(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;

    Ok((
        s,
        AttributeDefault {
            attribute_name: attribute_name.to_string(),
            attribute_value,
        },
    ))
}

fn env_float(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::EnvTypeFloat, char('0'))(s)
}

fn env_int(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::EnvTypeu64, char('1'))(s)
}

fn env_data(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::EnvTypeu64, char('2'))(s)
}

fn env_var_type(s: &str) -> IResult<&str, EnvType> {
    alt((env_float, env_int, env_data))(s)
}

/// Environment Variable Definitions
fn environment_variable(s: &str) -> IResult<&str, EnvironmentVariable> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("EV_")(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_type) = env_var_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brk_open(s)?;
    let (s, min) = complete::i64(s)?;
    let (s, _) = pipe(s)?;
    let (s, max) = complete::i64(s)?;
    let (s, _) = brk_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, unit) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, initial_value) = double(s)?;
    let (s, _) = ms1(s)?;
    let (s, ev_id) = complete::i64(s)?;
    let (s, _) = ms1(s)?;
    let (s, access_type) = access_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, access_nodes) = separated_list0(comma, access_node)(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        EnvironmentVariable {
            env_var_name,
            env_var_type,
            min,
            max,
            unit: unit.to_string(),
            initial_value,
            ev_id,
            access_type,
            access_nodes,
        },
    ))
}

fn environment_variable_data(s: &str) -> IResult<&str, EnvironmentVariableData> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("ENVVAR_DATA_")(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, data_size) = complete::u64(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        EnvironmentVariableData {
            env_var_name,
            data_size,
        },
    ))
}

fn new_symbols(s: &str) -> IResult<&str, Vec<Symbol>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("NS_ :")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending(s)?;
    let (s, symbols) = many0(symbol)(s)?;
    Ok((s, symbols))
}