use crate::network_system::network_node::{IpAddress, Status};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sentry {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status,
}

impl Sentry {
    pub fn new(ip_address: IpAddress, name: String, password: String, status: Status) -> Sentry {
        Sentry {
            ip_address,
            name,
            password,
            status,
        }
    }
}
