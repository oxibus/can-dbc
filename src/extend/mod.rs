mod signal;

/// DBC value
#[derive(Debug, Clone)]
pub struct Value {
    pub name: String,
    pub raw: u64,
    pub offset: f64,
    pub factor: f64,
    pub unit: String,
}

// TODO: decide if these are really needed
//       if added, the entire crate cannot use unsafe_code = "forbid" in Cargo.toml
// unsafe impl Send for Value {}
// unsafe impl Sync for Value {}

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
