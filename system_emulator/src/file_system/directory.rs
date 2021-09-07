use crate::file_system::file::File;
use crate::file_system::encryption::Encryption;

#[derive(Debug, PartialEq, Clone)]
pub struct Directory {
    name: String,
    directories: Vec<Box<Directory>>,
    files: Vec<File>,
    encryption: Encryption
}

impl Directory {
    pub fn new(name: String, directories: Vec<Box<Directory>>, files: Vec<File>, encryption: Encryption) -> Directory {
        Directory{name, directories, files, encryption}
    }
}