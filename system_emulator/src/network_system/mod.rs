pub mod devices;
pub mod network_node;
pub mod router;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::devices::{device::Device, nid::NID};
    use super::network_node::{IpAddress, NetworkNode, Status};
    use super::router::*;

    #[test]
    fn gen_rand_ips() {
        let mut router = Router::new(
            IpAddress::new(42690, 167, 0),
            "".into(),
            "".into(),
            Status::Online,
            HashMap::new(),
        );

        for i in 0..255 {
            assert_eq!(
                Ok(()),
                router.add_device(Device::NID(NID::new(
                    IpAddress::default(),
                    format!("NID_{}", i),
                    "".into(),
                    Status::Online
                )))
            )
        }

        assert_ne!(
            Ok(()),
            router.add_device(Device::NID(NID::new(
                IpAddress::default(),
                "NID_256".into(),
                "".into(),
                Status::Online
            )))
        );

        //this ensures that all of the device portion of the ips are unique
        let mut ips: Vec<u8> = router
            .get_devices()
            .iter()
            .map(|d| d.get_ip_address().device())
            .collect();
        assert_eq!(255, ips.len());
        ips.sort();
        ips.dedup();
        assert_eq!(255, ips.len());
    }
}
