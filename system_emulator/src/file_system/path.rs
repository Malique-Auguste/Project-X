#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Path {
    inner: Vec<String>
}

impl Path {
    pub fn new(inner: Vec<String>) -> Path {
        Path {inner}
    }
}