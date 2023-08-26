use ipnet::IpAddrRange;
use std::net::IpAddr;

struct OutputFormatter {
    input: String,
    network_address: IpAddr,
    broadcast_address: IpAddr,
    netmask: IpAddr,
    netmask_prefix: u8,
    hosts: IpAddrRange,
    first_host_address: Option<IpAddr>,
    last_host_address: Option<IpAddr>,
}

impl OutputFormatter {
    fn create(input: String, net: ipnet::IpNet) -> Self {
        Self {
            input,
            network_address: net.network(),
            broadcast_address: net.broadcast(),
            netmask: net.netmask(),
            netmask_prefix: net.prefix_len(),
            hosts: net.hosts(),
            first_host_address: net.hosts().next(),
            last_host_address: net.hosts().last(),
        }
    }
    fn print(&self) {
        println!("---");
        println!("YOUR INPUT         {}", self.input);
        println!("NETWORK ADDRESS    {}", self.network_address);
        println!("HOST ADDRESS RANGE {} ... {} (COUNT: {})", self.first_host(), self.last_host(), self.hosts.count());
        println!("BROADCAST ADDRESS  {}", self.broadcast_address);
        println!("NETMASK            {} (/{})", self.netmask, self.netmask_prefix);
    }

    fn first_host(&self) -> String {
        match self.first_host_address {
            Some(x) => return x.to_string(),
            None => return String::from("")
        };
    }

    fn last_host(&self) -> String {
        match self.last_host_address {
            Some(x) => return x.to_string(),
            None => return String::from("")
        };
    }
}

pub fn format(user_input: String, ipnetwork: ipnet::IpNet) {
    let formatter = OutputFormatter::create(user_input, ipnetwork);
    formatter.print();
}
