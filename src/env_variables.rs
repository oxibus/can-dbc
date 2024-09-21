#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::parser;
use crate::DBCObject;
use crate::{AccessNode, AccessType};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{self, line_ending, multispace0},
    combinator::value,
    multi::separated_list0,
    number::complete::double,
    IResult,
};

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariable {
    pub(crate) env_var_name: String,
    pub(crate) env_var_type: EnvType,
    pub(crate) min: i64,
    pub(crate) max: i64,
    pub(crate) unit: String,
    pub(crate) initial_value: f64,
    pub(crate) ev_id: i64,
    pub(crate) access_type: AccessType,
    pub(crate) access_nodes: Vec<AccessNode>,
}

impl DBCObject for EnvironmentVariable {
    fn dbc_string(&self) -> String {
        return format!(
            "EV_ {}: {} [{}|{}] \"{}\" {} {} {} {};\n",
            self.env_var_name,
            self.env_var_type.dbc_string(),
            self.min,
            self.max,
            self.unit,
            self.initial_value,
            self.ev_id,
            self.access_type.dbc_string(),
            self.access_nodes
                .clone()
                .into_iter()
                .map(|an| an.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("EV_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var_name) = parser::c_ident(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var_type) = EnvType::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::brk_open(s)?;
        let (s, min) = complete::i64(s)?;
        let (s, _) = parser::pipe(s)?;
        let (s, max) = complete::i64(s)?;
        let (s, _) = parser::brk_close(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, unit) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, initial_value) = double(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, ev_id) = complete::i64(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, access_type) = AccessType::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, access_nodes) = separated_list0(parser::comma, AccessNode::parse)(s)?;
        let (s, _) = parser::semi_colon(s)?;
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
        EnvironmentVariable::parse(def1).expect("Failed to parse environment variable");

    // Test parsing
    assert_eq!(env_var1, env_var);

    // Test generation
    assert_eq!(def1, env_var.dbc_string());
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariableData {
    pub(crate) env_var_name: String,
    pub(crate) data_size: u64,
}

impl DBCObject for EnvironmentVariableData {
    fn dbc_string(&self) -> String {
        return format!("ENVVAR_DATA_ {}: {};\n", self.env_var_name, self.data_size,);
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("ENVVAR_DATA_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var_name) = parser::c_ident(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, data_size) = complete::u64(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            EnvironmentVariableData {
                env_var_name,
                data_size,
            },
        ))
    }
}

#[test]
fn envvar_data_test() {
    let def = "ENVVAR_DATA_ SomeEnvVarData: 399;\n";
    let (_, envvar_data) = EnvironmentVariableData::parse(def).unwrap();
    let envvar_data_exp = EnvironmentVariableData {
        env_var_name: "SomeEnvVarData".to_string(),
        data_size: 399,
    };

    // Test parsing
    assert_eq!(envvar_data_exp, envvar_data);

    // Test generation
    assert_eq!(def, envvar_data.dbc_string());
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum EnvType {
    EnvTypeFloat,
    EnvTypeu64,
    EnvTypeData,
}

impl EnvType {
    fn env_float(s: &str) -> IResult<&str, EnvType> {
        value(EnvType::EnvTypeFloat, char('0'))(s)
    }

    fn env_int(s: &str) -> IResult<&str, EnvType> {
        value(EnvType::EnvTypeu64, char('1'))(s)
    }

    fn env_data(s: &str) -> IResult<&str, EnvType> {
        value(EnvType::EnvTypeu64, char('2'))(s)
    }
}

impl DBCObject for EnvType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::EnvTypeFloat => "0".to_string(),
            Self::EnvTypeu64 => "1".to_string(),
            Self::EnvTypeData => "".to_string(), // TODO determine what this value should enumerate to
        };
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((Self::env_float, Self::env_int, Self::env_data))(s)
    }
}
