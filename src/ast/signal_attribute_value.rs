// FIXME: not used!
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SignalAttributeValue {
    Text(String),
    Int(i64),
}
