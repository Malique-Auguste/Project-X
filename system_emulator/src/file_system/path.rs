#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    inner: Vec<String>
}

impl Path {
    pub fn new(inner: Vec<String>) -> Path {
        Path {inner}
    }
}