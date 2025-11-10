use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValue;
use crate::parser::{expect_empty, next, parse_next_inner_str, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefault {
    pub name: String,
    pub value: AttributeValue,
}

impl TryFrom<Pair<'_, Rule>> for AttributeDefault {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = value.as_rule();
        if !matches!(rule, Rule::ba_def_def | Rule::ba_def_def_rel) {
            return Err(DbcError::ExpectedRule(Rule::ba_def_def, rule));
        }

        let mut pairs = value.into_inner();
        let name = parse_next_inner_str(&mut pairs, Rule::attribute_name)?;
        let value = next(&mut pairs)?.try_into()?;
        expect_empty(&pairs)?;

        Ok(Self { name, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AttributeValue;
    use crate::test_helpers::*;

    #[test]
    fn attribute_default_test() {
        let def = r#"
BA_DEF_DEF_  "ZUV" "OAL";
"#;
        let exp = AttributeDefault {
            name: "ZUV".to_string(),
            value: AttributeValue::String("OAL".to_string()),
        };
        let val = test_into::<AttributeDefault>(def.trim_start(), Rule::ba_def_def);
        assert_eq!(val, exp);
    }
}
