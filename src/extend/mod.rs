mod signal;

use derive_getters::Getters;

/// DBC value
#[derive(Debug, Clone, Getters)]
pub struct Value {
    pub(crate) name: String,
    #[getter(rename = "raw_value")]
    #[getter(copy)]
    pub(crate) raw: u64,
    #[getter(copy)]
    pub(crate) offset: f64,
    #[getter(copy)]
    pub(crate) factor: f64,
    pub(crate) unit: String,
}

unsafe impl Send for Value {}
unsafe impl Sync for Value {}

impl Value {
    /// Get the value of the signal
    #[inline]
    pub fn value<T>(&self) -> T
    where
        T: num::NumCast + Default,
    {
        T::from(self.raw as f64 * self.factor + self.offset).unwrap_or_default()
    }
    /// Get the value and unit of the signal as a string
    #[inline]
    pub fn value_string<T>(&self) -> String
    where
        T: num::NumCast + Default + std::fmt::Display,
    {
        format!("{} {}", self.value::<T>(), self.unit)
    }
}
