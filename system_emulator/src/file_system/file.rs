use crate::file_system::encryption::Encryption;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
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

    pub fn content(&self) -> &String {
        match self.encryption {
            Encryption::None => &self.content,
            Encryption::Weak{key} => todo!(),
            Encryption::Medium{key, hash} => todo!(),
            Encryption::Strong{key1, key2, hash} => todo!()
        }
    }

    pub fn edit_content(&mut self, content: String) {
        self.content = content
    }

    pub fn edit_name(&mut self, new_name: String) {
        self.name = new_name
    }

    pub fn encryption(&self) -> &Encryption {
        &self.encryption
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FileType {
    Text
}

impl Default for FileType {
    fn default() -> FileType {
        FileType::Text
    }
}