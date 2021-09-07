use crate::file_system::encryption::Encryption;

#[derive(Debug, PartialEq, Clone)]
pub struct File {
    name: String,
    file_type: FileType,
    content: String,
    encryption: Encryption
}

impl File {
    pub fn new(name: String, file_type: FileType, content: String, encryption: Encryption) -> File {
        File {name, file_type, content, encryption}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileType {
    Text
}