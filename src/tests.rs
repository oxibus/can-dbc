#[cfg(test)]
mod tests {

    use std::convert::TryFrom;

    // use super::*;
    use crate::{DBCObject, Error, MessageId, SignalExtendedValueType, ValDescription, DBC};

    const SAMPLE_DBC: &str = r#"
VERSION "0.1"
NS_ :
	NS_DESC_
	CM_
	BA_DEF_
	BA_
	VAL_
	CAT_DEF_
	CAT_
	FILTER
	BA_DEF_DEF_
	EV_DATA_
	ENVVAR_DATA_
	SGTYPE_
	SGTYPE_VAL_
	BA_DEF_SGTYPE_
	BA_SGTYPE_
	SIG_TYPE_REF_
	VAL_TABLE_
	SIG_GROUP_
	SIG_VALTYPE_
	SIGTYPE_VALTYPE_
	BO_TX_BU_
	BA_DEF_REL_
	BA_REL_
	BA_DEF_DEF_REL_
	BU_SG_REL_
	BU_EV_REL_
	BU_BO_REL_
	SG_MUL_VAL_
BS_:
BU_: PC

BO_ 2000 WebData_2000: 4 Vector__XXX
 SG_ Signal_8 : 24|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_7 : 16|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_6 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_5 : 0|8@1+ (1,0) [0|255] "" Vector__XXX

BO_ 1840 WebData_1840: 4 PC
 SG_ Signal_4 : 24|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_3 : 16|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_2 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_1 : 0|8@1+ (1,0) [0|0] "" Vector__XXX

BO_ 3040 WebData_3040: 8 Vector__XXX
 SG_ Signal_6 m2 : 0|4@1+ (1,0) [0|15] "" Vector__XXX
 SG_ Signal_5 m3 : 16|8@1+ (1,0) [0|255] "kmh" Vector__XXX
 SG_ Signal_4 m3 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
 SG_ Signal_3 m3 : 0|4@1+ (1,0) [0|3] "" Vector__XXX
 SG_ Signal_2 m1 : 3|12@0+ (1,0) [0|4095] "Byte" Vector__XXX
 SG_ Signal_1 m0 : 0|4@1+ (1,0) [0|7] "Byte" Vector__XXX
 SG_ Switch M : 4|4@1+ (1,0) [0|3] "" Vector__XXX

EV_ Environment1: 0 [0|220] "" 0 6 DUMMY_NODE_VECTOR0 DUMMY_NODE_VECTOR2;
EV_ Environment2: 0 [0|177] "" 0 7 DUMMY_NODE_VECTOR1 DUMMY_NODE_VECTOR2;

ENVVAR_DATA_ SomeEnvVarData: 399;

CM_ BO_ 1840 "Some Message comment";
CM_ SG_ 1840 Signal_4 "asaklfjlsdfjlsdfgls
HH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv";
CM_ SG_ 5 TestSigLittleUnsigned1 "asaklfjlsdfjlsdfgls
=0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv";

BA_DEF_DEF_ "BusType" "AS";

BA_ "Attr" BO_ 2684354559 283;
BA_ "Attr" BO_ 2204433193 344;

VAL_ 2000 Signal_3 255 "NOP";

SIG_VALTYPE_ 2000 Signal_8 : 1;

"#;

    #[test]
    fn dbc_definition_test() {
        match DBC::try_from(SAMPLE_DBC) {
            Ok(dbc_content) => println!("DBC Content{:#?}", dbc_content),
            Err(e) => {
                match e {
                    Error::Nom(nom::Err::Incomplete(needed)) => {
                        eprintln!("Error incomplete input, needed: {:?}", needed)
                    }
                    Error::Nom(nom::Err::Error(error)) => {
                        eprintln!("Nom Error: {:?}", error);
                    }
                    Error::Nom(nom::Err::Failure(ctx)) => eprintln!("Failure {:?}", ctx),
                    Error::Incomplete(dbc, remaining) => eprintln!(
                        "Not all data in buffer was read {:#?}, remaining unparsed: {}",
                        dbc, remaining
                    ),
                    Error::MultipleMultiplexors => eprintln!("Multiple multiplexors defined"),
                }
                panic!("Failed to read DBC");
            }
        }
    }

    #[test]
    fn dbc_write_test() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        assert_eq!(SAMPLE_DBC, dbc_content.dbc_string());
    }

    #[test]
    fn lookup_signal_comment() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content
            .signal_comment(MessageId::Standard(1840), "Signal_4")
            .expect("Signal comment missing");
        assert_eq!(
            "asaklfjlsdfjlsdfgls\nHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv",
            comment
        );
    }

    #[test]
    fn lookup_signal_comment_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content.signal_comment(MessageId::Standard(1840), "Signal_2");
        assert_eq!(None, comment);
    }

    #[test]
    fn lookup_message_comment() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content
            .message_comment(MessageId::Standard(1840))
            .expect("Message comment missing");
        assert_eq!("Some Message comment", comment);
    }

    #[test]
    fn lookup_message_comment_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content.message_comment(MessageId::Standard(2000));
        assert_eq!(None, comment);
    }

    #[test]
    fn lookup_value_descriptions_for_signal() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let val_descriptions = dbc_content
            .value_descriptions_for_signal(MessageId::Standard(2000), "Signal_3")
            .expect("Message comment missing");

        let exp = vec![ValDescription {
            a: 255.0,
            b: "NOP".to_string(),
        }];
        assert_eq!(exp, val_descriptions);
    }

    #[test]
    fn lookup_value_descriptions_for_signal_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let val_descriptions =
            dbc_content.value_descriptions_for_signal(MessageId::Standard(2000), "Signal_2");
        assert_eq!(None, val_descriptions);
    }

    #[test]
    fn lookup_extended_value_type_for_signal() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let extended_value_type =
            dbc_content.extended_value_type_for_signal(MessageId::Standard(2000), "Signal_8");
        assert_eq!(
            extended_value_type,
            Some(&SignalExtendedValueType::IEEEfloat32Bit)
        );
    }

    #[test]
    fn lookup_extended_value_type_for_signal_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let extended_value_type =
            dbc_content.extended_value_type_for_signal(MessageId::Standard(2000), "Signal_1");
        assert_eq!(extended_value_type, None);
    }

    #[test]
    fn lookup_signal_by_name() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let signal = dbc_content.signal_by_name(MessageId::Standard(2000), "Signal_8");
        assert!(signal.is_some());
    }

    #[test]
    fn lookup_signal_by_name_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let signal = dbc_content.signal_by_name(MessageId::Standard(2000), "Signal_25");
        assert_eq!(signal, None);
    }

    #[test]
    fn lookup_multiplex_indicator_switch() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let multiplexor_switch = dbc_content.message_multiplexor_switch(MessageId::Standard(3040));
        assert!(multiplexor_switch.is_ok());
        assert!(multiplexor_switch.as_ref().unwrap().is_some());
        assert_eq!(multiplexor_switch.unwrap().unwrap().name(), "Switch");
    }

    #[test]
    fn lookup_multiplex_indicator_switch_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let multiplexor_switch = dbc_content.message_multiplexor_switch(MessageId::Standard(1840));
        assert!(multiplexor_switch.unwrap().is_none());
    }

    #[test]
    fn extended_message_id_raw() {
        let message_id = MessageId::Extended(2);
        assert_eq!(message_id.raw(), 2 | 1 << 31);
        let message_id = MessageId::Extended(2 ^ 29);
        assert_eq!(message_id.raw(), 2 ^ 29 | 1 << 31);
    }

    #[test]
    fn standard_message_id_raw() {
        let message_id = MessageId::Standard(2);
        assert_eq!(message_id.raw(), 2);
    }
}
