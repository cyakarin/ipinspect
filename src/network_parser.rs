use ipnet::IpNet;
use std::str::FromStr;
use std::net::IpAddr;
use std::fmt;
use std::error::Error;

pub struct NetworkParser {
    network: String
}

impl NetworkParser {
    pub fn new(network: String) -> Self {
        Self {
            network: network
        }
    }

    pub fn parse(&self) -> Result<IpNet, MyAddrParseError> {
        match self.netmask_bit() {
            Ok(bit) => {
                let v: Vec<&str> = self.network.split('/').collect();
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

    fn netmask_bit(&self) -> Result<u8, MyAddrParseError> {
        let v: Vec<&str> = self.network.split('/').collect();
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
