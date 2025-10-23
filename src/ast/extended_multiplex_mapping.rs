use derive_getters::Getters;

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplexMapping {
    pub min_value: u64,
    pub max_value: u64,
}
