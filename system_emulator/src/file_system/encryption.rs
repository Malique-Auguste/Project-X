#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Encryption {
    None,
    Weak { key: u8 },                       //basic cipher where 'key' is the offset value
    Medium {key: u8, hash: u8},             //encryption requiring 1 key. Hash needs to be the same to prove that no editing occurred 
    Strong {key1: u8, key2: u8, hash: u8}   //encryption requiring 2 keys. Hash needs to be the same to prove that no editing occurred 
}

impl Default for Encryption {
    fn default() -> Encryption {
        Encryption::None
    }
}