pub mod router;
pub mod switch;
pub mod computer;
pub mod device;
pub mod ip_address;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::file_system::directory::Directory;

    use super::device::Device;
    use super::computer::Computer;
    use super::switch::*;
    use super::router::*;
    use super::ip_address::*;

    #[test]
    fn rand_ips_generated() {
        let mut switch = Switch::new(IpAddress::new(2431, 152, 0), HashMap::new());
        let mut router = Router::new(IpAddress::new(2431, 0, 0), HashMap::new());

        assert_ne!(switch.gen_new_ip_address(), switch.gen_new_ip_address());
        assert_ne!(switch.gen_new_ip_address(), switch.gen_new_ip_address());

        assert_ne!(router.gen_new_ip_address(), router.gen_new_ip_address());
        assert_ne!(router.gen_new_ip_address(), router.gen_new_ip_address());
    }
}