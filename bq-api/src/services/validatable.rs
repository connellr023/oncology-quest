/// Trait for types that can be validated.
pub(super) trait Validatable {
    /// Validates the current instance.
    ///
    /// Returns `true` if the instance is valid, `false` otherwise.
    fn is_valid(&self) -> bool;
}