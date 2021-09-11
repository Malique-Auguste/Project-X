use crate::file_system::encryption::Encryption;
use crate::file_system::file::File;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Directory {
    pub name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
    encryption: Encryption,
}

impl Directory {
    pub fn new(
        name: String,
        directories: Vec<Directory>,
        files: Vec<File>,
        encryption: Encryption,
    ) -> Directory {
        Directory {
            name,
            directories,
            files,
            encryption,
        }
    }

    pub fn get_dir(&self, name: &str) -> Option<&Directory> {
        self.directories.iter().find(|dir| dir.name == name)
    }

    pub fn get_file(&self, name: &str) -> Option<&File> {
        self.files.iter().find(|f| f.name == name)
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
