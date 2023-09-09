use ipnet::IpNet;
use std::str::FromStr;
use std::net::IpAddr;
use std::fmt;
use std::error::Error;

pub struct IpInspector {
    input_network: String,
    network: IpNet
}

impl IpInspector {
    pub fn build(network: String) -> Result<Self, MyAddrParseError> {
        let parsed_network: IpNet = match IpInspector::parse(&network) {
            Ok(ipnet) => { ipnet }
            Err(e) => { return Err(e); }
        };
        Ok(Self { input_network: network, network: parsed_network })
    }

    pub fn print_for_human(&self) {
        println!("---");
        println!("YOUR INPUT         {}", self.input_network);
        println!("NETWORK ADDRESS    {}", self.network_address());
        println!("HOST ADDRESS RANGE {} ... {} (COUNT: {})", self.first_host_address(), self.last_host_address(), self.hosts_count());
        if self.network_address().is_ipv4() {
            println!("BROADCAST ADDRESS  {}", self.broadcast_address());
        } else {
            println!("BROADCAST ADDRESS  {}", "Nothing. MulticastAddress is used instead of it.");
        }
        println!("NETMASK            {} (/{})", self.netmask(), self.netmask_prefix());
    }

    fn parse(network: &str) -> Result<IpNet, MyAddrParseError> {
        match IpInspector::netmask_bit(network) {
            Ok(bit) => {
                let v: Vec<&str> = network.split('/').collect();
                let net: String = String::from(v[0]) + "/" + &bit.to_string();
                match IpNet::from_str(&net) {
                    Ok(parsed_network) => {
                        return Ok(parsed_network);
                    }
                    Err(_) => {
                        return Err(MyAddrParseError::new(String::from("Invalid IP network")));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }

        }
    }

    fn netmask_bit(network: &str) -> Result<u8, MyAddrParseError> {
        let v: Vec<&str> = network.split('/').collect();
        if v.len() != 2 {
            return Err(MyAddrParseError::new(String::from("Invalid IP network")));
        }
        if v[1].contains('.') || v[1].contains(':') {
            match IpAddr::from_str(v[1]) {
                Ok(mask) => {
                    match ipnetwork::ip_mask_to_prefix(mask) {
                        Ok(bit) => {
                            return Ok(bit);
                        }
                        Err(_) => {
                            return Err(MyAddrParseError::new(String::from("invalid netmask")));
                        }
                    }
                }
                Err(_) => {
                    return Err(MyAddrParseError::new(String::from("invalid netmask")));
                }
            }
        } else {
            match v[1].parse::<u8>() {
                Ok(bit) => { return Ok(bit); }
                Err(_e) => { return Err(MyAddrParseError::new(String::from("invalid netmask"))); }
            }
        }
    }

    fn network_address(&self) -> IpAddr {
        self.network.network()
    }

    fn broadcast_address(&self) -> IpAddr {
        self.network.broadcast()
    }

    fn netmask(&self) -> IpAddr {
        self.network.netmask()
    }

    fn netmask_prefix(&self) -> u8 {
        self.network.prefix_len()
    }

    fn first_host_address(&self) -> String {
        match self.network.hosts().next() {
            Some(x) => return x.to_string(),
            None => return String::from("")
        };
    }

    fn last_host_address(&self) -> String {
        match self.network.hosts().last() {
            Some(x) => return x.to_string(),
            None => return String::from("")
        };
    }

    fn hosts_count(&self) -> String {
        let max_mask_bits = if self.network_address().is_ipv4() { 32 } else { 128 };
        if usize::BITS > (max_mask_bits - self.netmask_prefix()).into() {
            return self.network.hosts().count().to_string()
        } else {
            return String::from("TOO MANY")
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyAddrParseError {
    message: String
}

impl fmt::Display for MyAddrParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.message)
    }
}

impl MyAddrParseError {
    fn new(error: String) -> Self {
        MyAddrParseError {
            message: error
        }
    }
}

impl Error for MyAddrParseError {}
