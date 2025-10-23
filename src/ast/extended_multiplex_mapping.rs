#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplexMapping {
    pub min_value: u64,
    pub max_value: u64,
}
