#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Encryption {
    None,
    Weak,
    Medium,
    Strong
}