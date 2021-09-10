use crate::network_system::network_node::{IpAddress, Status};


/*
NID stands for a Non Interactable Device such as a game console, printer or bluetooth speaker
They exist to make the world feel more alive
*/
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NID {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status
}

impl NID {
    pub fn new(ip_address: IpAddress, name: String, password: String, status: Status) -> NID {
        NID{ip_address, name, password, status}
    }
}