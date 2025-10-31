// FIXME: not used!
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValueType {
    Int(i64, i64),
    Hex(i64, i64),
    Float(f64, f64),
    String,
    Enum(Vec<String>),
}
