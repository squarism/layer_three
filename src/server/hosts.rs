// represents a hosts file, what each server can lookup
// we are not solving DNS here

use std::net::IpAddr;

#[allow(dead_code)]
pub struct Host {
    pub host: String,
    pub ip: IpAddr,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hosts_file() {
        let host1 = Host {
            host: "box1".to_owned(),
            ip: "1.2.3.4".parse().unwrap(),
        };

        let host2 = Host {
            host: "box2".to_owned(),
            ip: "2.3.4.5".parse().unwrap(),
        };

        let hosts_file = vec![host1, host2];
        assert_eq!(hosts_file.len(), 2);
    }
}
