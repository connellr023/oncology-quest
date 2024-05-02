pub trait Validatable {
    fn validate(&self) -> bool;
}