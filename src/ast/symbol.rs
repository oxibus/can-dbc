use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(pub String);

impl TryFrom<Pair<'_, Rule>> for Symbol {
    type Error = DbcError;

    /// Parse new symbols: NS_ : symbol1 symbol2 ...
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(Self(validated(value, Rule::ident)?.as_str().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::collect_all;
    use crate::test_helpers::*;

    #[test]
    fn new_symbols_test() {
        let def = "
NS_ :
    NS_DESC_
    CM_
    BA_DEF_
";
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
}
