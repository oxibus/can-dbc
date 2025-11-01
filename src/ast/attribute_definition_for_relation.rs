use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefinitionForRelation {
    name: String,
    value_spec: String,
}

impl TryFrom<Pair<'_, Rule>> for AttributeDefinitionForRelation {
    type Error = DbcError;

    /// Parse attribute definition for relation: `BA_DEF_REL_ rel_object_type attribute_name attribute_type;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let inner_pairs = validated_inner(value, Rule::ba_def_rel)?;
        let mut value_spec = String::new();
        let mut name = None;

        // Process all pairs
        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::rel_object_type => {
                    // rel_object_type is not included in value_spec, just skip it
                }
                Rule::attribute_name => {
                    // Extract the name (inner string from quoted string)
                    name = Some(inner_str(pair.clone()));
                    // Include the full attribute_name (with quotes) in value_spec
                    if !value_spec.is_empty() {
                        value_spec.push(' ');
                    }
                    value_spec.push_str(pair.as_str());
                }
                Rule::attribute_type_int
                | Rule::attribute_type_hex
                | Rule::attribute_type_float
                | Rule::attribute_type_string
                | Rule::attribute_type_enum => {
                    if !value_spec.is_empty() {
                        value_spec.push(' ');
                    }
                    value_spec.push_str(pair.as_str());
                }
                v => return Err(DbcError::UnknownRule(v)),
            }
        }

        Ok(Self {
            name: name.ok_or(DbcError::NoMoreRules)?,
            value_spec,
        })
    }
}
