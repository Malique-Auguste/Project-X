/*
Representation of a path to a file or directory.
Each member of inner is the name of the subsequent directory to traverse before reaching the file/directory.
*/

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Path {
    inner: Vec<String>
}

impl Path {
    pub fn new(inner: Vec<String>) -> Path {
        Path {inner}
    }
}