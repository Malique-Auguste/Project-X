use crate::file_system::file::File;
use crate::file_system::encryption::Encryption;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
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

    pub fn directories(&self) -> &Vec<Box<Directory>> {
        &self.directories
    }

    pub fn directories_mut(&mut self) -> &mut Vec<Box<Directory>> {
        &mut self.directories
    }

    pub fn encryption(&self) -> &Encryption {
        &self.encryption
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn files_mut(&mut self) -> &mut Vec<File> {
        &mut self.files
    }
}