use crate::file_system::{directory::Directory, file::File, path::Path};
use crate::network_system::network_node::{IpAddress, Status};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Computer {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status,
    current_path: Path,
    storage: Directory,
}

impl Computer {
    pub fn new(
        ip_address: IpAddress,
        name: String,
        password: String,
        status: Status,
        current_path: Path,
        storage: Directory,
    ) -> Computer {
        Computer {
            ip_address,
            name,
            password,
            status,
            current_path,
            storage,
        }
    }
}
