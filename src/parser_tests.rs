#![allow(clippy::needless_raw_string_hashes)]

use std::fmt::Debug;

use can_dbc_pest::*;

use crate::parser::*;
use crate::*;

/// Helper function to parse a snippet with a specific rule, returning just the rule's inner content already unwrapped
fn parse(input: &str, rule: Rule) -> DbcResult<Pair<'_, Rule>> {
    let pairs = DbcParser::parse(rule, input)?;
    Ok(pairs.into_iter().next().unwrap())
}

fn span(input: &str, rule: Rule) -> &str {
    let pair = parse(input, rule).unwrap();
    pair.as_span().as_str()
}

fn test_into<'a, T>(pair: &Pair<'a, Rule>) -> T
where
    T: TryFrom<Pair<'a, Rule>>,
    <T as TryFrom<Pair<'a, Rule>>>::Error: Debug,
{
    pair.clone().try_into().unwrap_or_else(|e| {
        panic!("{e:?}:\n{pair:#?}");
    })
}

#[test]
fn c_ident_test() {
    assert_eq!(span("EALL_DUSasb18 ", Rule::ident), "EALL_DUSasb18");
    assert_eq!(span("_EALL_DUSasb18 ", Rule::ident), "_EALL_DUSasb18");
    assert!(parse("3EALL_DUSasb18 ", Rule::ident).is_err());
}

#[test]
fn c_ident_vec_test() {
    let def = "FZHL_DUSasb18 ";
    let pair = parse(def, Rule::ident).unwrap();
    let val = vec![Symbol(pair.as_str().to_string())];
    assert_eq!(val, vec![Symbol("FZHL_DUSasb18".to_string())]);

    let def = "FZHL_DUSasb19,xkask_3298 ";
    let pair = parse(def, Rule::ident).unwrap();
    let val = vec![Symbol(pair.as_str().to_string())];
    assert_eq!(val, vec![Symbol("FZHL_DUSasb19".to_string())],);
}

#[test]
fn char_string_test() {
    let def = "\"ab\x00\x7f\"";
    let pair = parse(def, Rule::quoted_str).unwrap();
    let val = pair.as_str();
    assert_eq!(val, "\"ab\x00\x7f\"");
}

#[test]
fn signal_test() {
    let def = r#"
SG_ NAME : 3|2@1- (1,0) [0|0] "x" UFA
"#;

    let val: Signal = parse(def.trim_start(), Rule::signal)
        .unwrap()
        .try_into()
        .unwrap();

    let exp = Signal {
        name: "NAME".to_string(),
        start_bit: 3,
        size: 2,
        byte_order: ByteOrder::LittleEndian,
        value_type: ValueType::Signed,
        factor: 1.0,
        offset: 0.0,
        min: 0.0,
        max: 0.0,
        unit: "x".to_string(),
        multiplexer_indicator: MultiplexIndicator::Plain,
        receivers: vec!["UFA".to_string()],
    };

    assert_eq!(val, exp);
}

#[test]
fn byte_order_test() {
    let val: ByteOrder = parse("0", Rule::big_endian).unwrap().try_into().unwrap();
    assert_eq!(val, ByteOrder::BigEndian);

    let val: ByteOrder = parse("1", Rule::little_endian).unwrap().try_into().unwrap();
    assert_eq!(val, ByteOrder::LittleEndian);
}

#[test]
fn multiplexer_indicator_test() {
    let val: MultiplexIndicator = "m34920".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::MultiplexedSignal(34920));

    let val: MultiplexIndicator = "M".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::Multiplexor);

    // Empty string is not a valid multiplexer indicator, so we skip this test
    // let val: MultiplexIndicator = "".try_into().unwrap();
    // assert_eq!(val, MultiplexIndicator::Plain);

    let val: MultiplexIndicator = "m8M".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::MultiplexorAndMultiplexedSignal(8));
}

#[test]
fn value_type_test() {
    let val: ValueType = parse("-", Rule::signed_type).unwrap().try_into().unwrap();
    assert_eq!(val, ValueType::Signed);

    let val: ValueType = parse("+", Rule::unsigned_type).unwrap().try_into().unwrap();
    assert_eq!(val, ValueType::Unsigned);
}

#[test]
fn message_definition_test() {
    let def = "\r\n\r\nSG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n";

    let val: Signal = parse(def.trim_start(), Rule::signal)
        .unwrap()
        .try_into()
        .unwrap();
    let exp = Signal {
        name: "BasL2".to_string(),
        start_bit: 3,
        size: 2,
        byte_order: ByteOrder::BigEndian,
        value_type: ValueType::Signed,
        factor: 1.0,
        offset: 0.0,
        min: 0.0,
        max: 0.0,
        unit: "x".to_string(),
        multiplexer_indicator: MultiplexIndicator::Plain,
        receivers: vec!["DFA_FUS".to_string()],
    };
    assert_eq!(val, exp);

    // this is disabled because message and signals are not currently parsed together

    //     let def = r#"
    // BO_ 1 MCA_A1: 6 MFA
    // SG_ ABC_1 : 9|2@1+ (1,0) [0|0] "x" XYZ_OUS
    // SG_ BasL2 : 3|2@0- (1,0) [0|0] "x" DFA_FUS
    //  x"#;
    //
    //     let val: Message = parse(def, Rule::message).unwrap().try_into().unwrap();
    //     let exp = Message {
    //         id: MessageId::Standard(1),
    //         name: "MCA_A1".to_string(),
    //         size: 6,
    //         transmitter: Transmitter::NodeName("MFA".to_string()),
    //         signals: vec![
    //             Signal {
    //                 name: "ABC_1".to_string(),
    //                 start_bit: 9,
    //                 size: 2,
    //                 byte_order: ByteOrder::LittleEndian,
    //                 value_type: ValueType::Unsigned,
    //                 factor: 1.0,
    //                 offset: 0.0,
    //                 min: 0.0,
    //                 max: 0.0,
    //                 unit: "x".to_string(),
    //                 multiplexer_indicator: MultiplexIndicator::Plain,
    //                 receivers: vec!["XYZ_OUS".to_string()],
    //             },
    //             Signal {
    //                 name: "BasL2".to_string(),
    //                 start_bit: 3,
    //                 size: 2,
    //                 byte_order: ByteOrder::BigEndian,
    //                 value_type: ValueType::Signed,
    //                 factor: 1.0,
    //                 offset: 0.0,
    //                 min: 0.0,
    //                 max: 0.0,
    //                 unit: "x".to_string(),
    //                 multiplexer_indicator: MultiplexIndicator::Plain,
    //                 receivers: vec!["DFA_FUS".to_string()],
    //             },
    //         ],
    //     };
    //     assert_eq!(val, exp);
}

#[test]
fn signal_comment_test() {
    let def = r#"
CM_ SG_ 193 KLU_R_X "This is a signal comment test";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Standard(193),
        name: "KLU_R_X".to_string(),
        comment: "This is a signal comment test".to_string(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn message_definition_comment_test() {
    let def = r#"
CM_ BO_ 34544 "Some Message comment";
"#;
    let exp = Comment::Message {
        id: MessageId::Standard(34544),
        comment: "Some Message comment".to_string(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn node_comment_test() {
    let def = r#"
CM_ BU_ network_node "Some network node comment";
"#;
    let exp = Comment::Node {
        name: "network_node".to_string(),
        comment: "Some network node comment".to_string(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn env_var_comment_test() {
    let def = r#"
CM_ EV_ ENVXYZ "Some env var name comment";
"#;
    let exp = Comment::EnvVar {
        name: "ENVXYZ".to_string(),
        comment: "Some env var name comment".to_string(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn signal_comment_with_escaped_characters_test() {
    let def = r#"
CM_ SG_ 2147548912 FooBar "Foo\\ \n \"Bar\"";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Extended(65264),
        name: "FooBar".to_string(),
        comment: r#"Foo\\ \n \"Bar\""#.to_string(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn empty_signal_comment_test() {
    let def = r#"
CM_ SG_ 2147548912 FooBar "";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Extended(65264),
        name: "FooBar".to_string(),
        comment: String::new(),
    };
    let pair = parse(def.trim_start(), Rule::comment).unwrap();
    let val = test_into::<Comment>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn value_description_for_signal_test() {
    let def = r#"
VAL_ 837 UF_HZ_OI 255 "NOP";
"#;
    let exp = ValueDescription::Signal {
        message_id: MessageId::Standard(837),
        name: "UF_HZ_OI".to_string(),
        value_descriptions: vec![ValDescription {
            id: 255,
            description: "NOP".to_string(),
        }],
    };
    let pair = parse(def.trim_start(), Rule::value_table_def).unwrap();
    let val = test_into::<ValueDescription>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn value_description_for_env_var_test() {
    let def = r#"
VAL_ MY_ENV_VAR 255 "NOP";
"#;
    let exp = ValueDescription::EnvironmentVariable {
        name: "MY_ENV_VAR".to_string(),
        value_descriptions: vec![ValDescription {
            id: 255,
            description: "NOP".to_string(),
        }],
    };
    let pair = parse(def.trim_start(), Rule::value_table_def).unwrap();
    let val = test_into::<ValueDescription>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn environment_variable_test() {
    let def = r#"
EV_ IUV: 0 [-22|20] "mm" 3 7 DUMMY_NODE_VECTOR0 VECTOR_XXX;
"#;
    let exp = EnvironmentVariable {
        name: "IUV".to_string(),
        typ: EnvType::Integer,
        min: -22,
        max: 20,
        unit: "mm".to_string(),
        initial_value: 3,
        ev_id: 7,
        access_type: AccessType::DummyNodeVector0,
        access_nodes: vec![AccessNode::Name("VECTOR_XXX".to_string())],
    };
    let pair = parse(def.trim_start(), Rule::environment_variable).unwrap();
    let val = test_into::<EnvironmentVariable>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn network_node_attribute_value_test() {
    let def = r#"
BA_ "AttrName" BU_ NodeName 12;
"#;
    let exp = AttributeValueForObject {
        name: "AttrName".to_string(),
        value: AttributeValuedForObjectType::NetworkNode(
            "NodeName".to_string(),
            AttributeValue::Uint(12),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn integer_and_float_attributes() {
    // attribute with a fractional part ".0" is parsed as Double
    let def = r#"
BA_ "Attribute" BU_ NodeName 12;
"#;
    let exp = AttributeValueForObject {
        name: "Attribute".to_string(),
        value: AttributeValuedForObjectType::NetworkNode(
            "NodeName".to_string(),
            AttributeValue::Uint(12),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);

    // negative attribute value without a fractional part is parsed as I64
    let def = r#"
BA_ "Attribute" BU_ NodeName -12;
"#;
    let exp = AttributeValueForObject {
        name: "Attribute".to_string(),
        value: AttributeValuedForObjectType::NetworkNode(
            "NodeName".to_string(),
            AttributeValue::Int(-12),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);

    // positive attribute value without a fractional part is parsed as I64
    let def = r#"
BA_ "Attribute" BU_ NodeName 12;
"#;
    let exp = AttributeValueForObject {
        name: "Attribute".to_string(),
        value: AttributeValuedForObjectType::NetworkNode(
            "NodeName".to_string(),
            AttributeValue::Uint(12),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);

    // positive attribute value without a fractional part is parsed as I64
    let def = r#"
BA_ "Attribute" BU_ NodeName 12.1;
"#;
    let exp = AttributeValueForObject {
        name: "Attribute".to_string(),
        value: AttributeValuedForObjectType::NetworkNode(
            "NodeName".to_string(),
            AttributeValue::Double(12.1),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn message_definition_attribute_value_test() {
    let def = r#"
BA_ "AttrName" BO_ 298 13;
"#;
    let exp = AttributeValueForObject {
        name: "AttrName".to_string(),
        value: AttributeValuedForObjectType::MessageDefinition(
            MessageId::Standard(298),
            Some(AttributeValue::Uint(13)),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn signal_attribute_value_test() {
    let def = r#"
BA_ "AttrName" SG_ 198 SGName 13;
"#;
    let exp = AttributeValueForObject {
        name: "AttrName".to_string(),
        value: AttributeValuedForObjectType::Signal(
            MessageId::Standard(198),
            "SGName".to_string(),
            AttributeValue::Uint(13),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn env_var_attribute_value_test() {
    let def = r#"
BA_ "AttrName" EV_ EvName "CharStr";
"#;
    let exp = AttributeValueForObject {
        name: "AttrName".to_string(),
        value: AttributeValuedForObjectType::EnvVariable(
            "EvName".to_string(),
            AttributeValue::String("CharStr".to_string()),
        ),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn raw_attribute_value_test() {
    let def = r#"
BA_ "AttrName" "RAW";
"#;
    let exp = AttributeValueForObject {
        name: "AttrName".to_string(),
        value: AttributeValuedForObjectType::Raw(AttributeValue::String("RAW".to_string())),
    };
    let pair = parse(def.trim_start(), Rule::attr_value).unwrap();
    let val = test_into::<AttributeValueForObject>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn new_symbols_test() {
    let def = r#"
NS_ :
                NS_DESC_
                CM_
                BA_DEF_

            "#;
    let exp = vec![
        Symbol("NS_DESC_".to_string()),
        Symbol("CM_".to_string()),
        Symbol("BA_DEF_".to_string()),
    ];
    let pair = parse(def.trim_start(), Rule::new_symbols).unwrap();
    let mut pairs = pair.into_inner();
    let val: Vec<Symbol> = collect_all(&mut pairs).unwrap();
    assert_eq!(val, exp);
}

#[test]
fn network_node_test() {
    let def = r#"
BU_: ZU XYZ ABC OIU
"#;
    let exp = vec![
        Node("ZU".to_string()),
        Node("XYZ".to_string()),
        Node("ABC".to_string()),
        Node("OIU".to_string()),
    ];
    let pair = parse(def.trim_start(), Rule::nodes).unwrap();
    let val: Vec<Node> = collect_all(&mut pair.into_inner()).unwrap();
    assert_eq!(val, exp);
}

#[test]
fn empty_network_node_test() {
    let def = r#"
BU_:
"#;
    let pair = parse(def.trim_start(), Rule::nodes).unwrap();
    let val: Vec<Node> = collect_all(&mut pair.into_inner()).unwrap();
    assert_eq!(val, vec![]);
}

#[test]
fn envvar_data_test() {
    let def = r#"
ENVVAR_DATA_ SomeEnvVarData: 399;
"#;
    let exp = EnvironmentVariableData {
        env_var_name: "SomeEnvVarData".to_string(),
        data_size: 399,
    };
    let pair = parse(def.trim_start(), Rule::env_var_data).unwrap();
    let val = test_into::<EnvironmentVariableData>(&pair);
    assert_eq!(val, exp);
}

// #[test]
// fn signal_type_test() {
//     // TODO: Fix rule name - signal_value_type doesn't exist
//     let def = r#"
// SGTYPE_ signal_type_name: 1024@1+ (5,2) [1|3] "unit" 2.0 val_table;
// "#;
//
//     let exp = SignalType {
//         name: "signal_type_name".to_string(),
//         signal_size: 1024,
//         byte_order: ByteOrder::LittleEndian,
//         value_type: ValueType::Unsigned,
//         factor: 5.0,
//         offset: 2.0,
//         min: 1.0,
//         max: 3.0,
//         unit: "unit".to_string(),
//         default_value: 2.0,
//         value_table: "val_table".to_string(),
//     };
//
//     let pair = parse(def.trim_start(), Rule::signal_value_type).unwrap();
//     let val = SignalExtendedValueTypeList::parse(pair).unwrap();
//     // assert_eq!(val, exp);
// }

#[test]
fn signal_groups_test() {
    let def = r#"
SIG_GROUP_ 23 X_3290 1 : A_b XY_Z;
"#;

    let exp = SignalGroups {
        message_id: MessageId::Standard(23),
        name: "X_3290".to_string(),
        repetitions: 1,
        signal_names: vec!["A_b".to_string(), "XY_Z".to_string()],
    };

    let pair = parse(def.trim_start(), Rule::signal_group).unwrap();
    let val = test_into::<SignalGroups>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn attribute_default_test() {
    let def = r#"
BA_DEF_DEF_  "ZUV" "OAL";
"#;
    let exp = AttributeDefault {
        name: "ZUV".to_string(),
        value: AttributeValue::String("OAL".to_string()),
    };
    let pair = parse(def.trim_start(), Rule::ba_def_def).unwrap();
    let val = test_into::<AttributeDefault>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn attribute_value_f64_test() {
    let def = "80.0";
    let pair = parse(def.trim_start(), Rule::number).unwrap();
    let val = pair.as_str().parse::<f64>().unwrap();
    assert!((val - 80.0).abs() < f64::EPSILON);
}

#[test]
fn attribute_definition_bo_test() {
    // TODO: Fix rule name - ba_def_def doesn't exist
    // let def_bo = r#"
    // BA_DEF_ BO_ "BaDef1BO" INT 0 1000000;
    // "#;
    // let pair = parse(def_bo.trim_start(), Rule::ba_def_def).unwrap();
    // let val = AttributeDefinition::parse(pair).unwrap();
    // let exp = AttributeDefinition::Message(r#""BaDef1BO" INT 0 1000000"#.to_string());
    // assert_eq!(val, exp);
}

#[test]
fn attribute_definition_bo_txt_test() {
    let def = r#"
BA_DEF_ BO_  "GenMsgSendType" STRING ;
"#;
    let pair = parse(def.trim_start(), Rule::attr_def).unwrap();
    let val = test_into::<AttributeDefinition>(&pair);
    let exp = AttributeDefinition::Message(r#""GenMsgSendType" STRING"#.to_string());
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_bu_test() {
    let def = r#"
BA_DEF_ BU_ "BuDef1BO" INT 0 1000000;
"#;
    let pair = parse(def.trim_start(), Rule::attr_def).unwrap();
    let val = test_into::<AttributeDefinition>(&pair);
    let exp = AttributeDefinition::Node(r#""BuDef1BO" INT 0 1000000"#.to_string());
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_sg_test() {
    let def = r#"
BA_DEF_ SG_ "SgDef1BO" INT 0 1000000;
"#;
    let pair = parse(def.trim_start(), Rule::attr_def).unwrap();
    let val = test_into::<AttributeDefinition>(&pair);
    let exp = AttributeDefinition::Signal(r#""SgDef1BO" INT 0 1000000"#.to_string());
    assert_eq!(val, exp);
}

// #[test]
// fn attribute_definition_ev_test() {
//     // TODO: Fix rule name - ba_def_def doesn't exist
//     let def_env_var = r#"
// BA_DEF_ EV_ "EvDef1BO" INT 0 1000000;
// "#;
//     let pair = parse(def_env_var.trim_start(), Rule::ba_def_def).unwrap();
//     let val = test_into::<AttributeDefinition>(&pair);
//     let exp = AttributeDefinition::EnvironmentVariable(r#""EvDef1BO" INT 0 1000000"#.to_string());
//     assert_eq!(val, exp);
// }

#[test]
fn version_test() {
    let def = r#"
VERSION "HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///"
"#;
    let exp = Version("HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///".to_string());
    let pair = parse(def.trim_start(), Rule::version).unwrap();
    let val = test_into::<Version>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn message_transmitters_test() {
    let def = r#"
BO_TX_BU_ 12345 : XZY,ABC;
"#;
    let exp = MessageTransmitter {
        message_id: MessageId::Standard(12345),
        transmitter: vec![
            Transmitter::NodeName("XZY".to_string()),
            Transmitter::NodeName("ABC".to_string()),
        ],
    };
    let pair = parse(def.trim_start(), Rule::message_transmitter).unwrap();
    let val = test_into::<MessageTransmitter>(&pair);
    assert_eq!(val, exp);

    // Same as above, but without space before the colon
    let def = r#"
BO_TX_BU_ 12345 :XZY,ABC;
"#;
    let pair = parse(def.trim_start(), Rule::message_transmitter).unwrap();
    let val = test_into::<MessageTransmitter>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn value_description_test() {
    let def = r#"
2 "ABC"
"#;
    let exp = ValDescription {
        id: 2,
        description: "ABC".to_string(),
    };
    let pair = parse(def.trim_start(), Rule::table_value_description).unwrap();
    let val = ValDescription::try_from(pair).unwrap();
    assert_eq!(val, exp);
}

#[test]
fn val_table_test() {
    let def = r#"
VAL_TABLE_ Tst 2 "ABC" 1 "Test A" ;
"#;
    let exp = ValueTable {
        name: "Tst".to_string(),
        descriptions: vec![
            ValDescription {
                id: 2,
                description: "ABC".to_string(),
            },
            ValDescription {
                id: 1,
                description: "Test A".to_string(),
            },
        ],
    };
    let pair = parse(def.trim_start(), Rule::value_table).unwrap();
    let val = test_into::<ValueTable>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn val_table_no_space_preceding_comma_test() {
    let def = r#"
VAL_TABLE_ Tst 2 "ABC";
"#;
    let exp = ValueTable {
        name: "Tst".to_string(),
        descriptions: vec![ValDescription {
            id: 2,
            description: "ABC".to_string(),
        }],
    };
    let pair = parse(def.trim_start(), Rule::value_table).unwrap();
    let val = test_into::<ValueTable>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn extended_multiplex_test() {
    // simple mapping
    let def = r#"
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1-1;
"#;
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1,
            max_value: 1,
        }],
    };
    let pair = parse(def.trim_start(), Rule::sg_mul_val).unwrap();
    let val = test_into::<ExtendedMultiplex>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn extended_multiplex_range_test() {
    // range mapping
    let def = r#"
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1568-2568;
"#;
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1568,
            max_value: 2568,
        }],
    };
    let pair = parse(def.trim_start(), Rule::sg_mul_val).unwrap();
    let val = test_into::<ExtendedMultiplex>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn extended_multiplex_mult_test() {
    // multiple mappings
    let def = r#"
SG_MUL_VAL_ 2147483650 muxed_B_5 MUX_B 5-5, 16-24;
"#;
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
    let pair = parse(def.trim_start(), Rule::sg_mul_val).unwrap();
    let val = test_into::<ExtendedMultiplex>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn sig_val_type_test() {
    let def = r#"
SIG_VALTYPE_ 2000 Signal_8 : 1;
"#;
    let exp = SignalExtendedValueTypeList {
        message_id: MessageId::Standard(2000),
        signal_name: "Signal_8".to_string(),
        signal_extended_value_type: SignalExtendedValueType::IEEEfloat32Bit,
    };

    let pair = parse(def.trim_start(), Rule::signal_value_type).unwrap();
    let val = test_into::<SignalExtendedValueTypeList>(&pair);
    assert_eq!(val, exp);
}

#[test]
fn standard_message_id_test() {
    let pair = parse("2", Rule::message_id).unwrap();
    let val = MessageId::try_from(pair).unwrap();
    assert_eq!(val, MessageId::Standard(2));
}

#[test]
fn extended_low_message_id_test() {
    let s = (2u32 | 1 << 31).to_string();
    let pair = parse(&s, Rule::message_id).unwrap();
    let val = MessageId::try_from(pair).unwrap();
    assert_eq!(val, MessageId::Extended(2));
}

#[test]
fn extended_message_id_test() {
    let s = (0x1FFF_FFFF_u32 | 1 << 31).to_string();
    let pair = parse(&s, Rule::message_id).unwrap();
    let val = MessageId::try_from(pair).unwrap();
    assert_eq!(val, MessageId::Extended(0x1FFF_FFFF));
}

#[test]
fn extended_message_id_test_max_29bit() {
    let s = u32::MAX.to_string();
    let pair = parse(&s, Rule::message_id).unwrap();
    let val = MessageId::try_from(pair).unwrap();
    assert_eq!(val, MessageId::Extended(0x1FFF_FFFF));
}
