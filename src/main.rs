mod output_formatter;

use clap::Parser;
use std::str::FromStr;
use ipnet::IpNet;

#[derive(Parser)]
struct Cli {
    ipnetwork: String
}

fn main() {
    let args = Cli::parse();
    let input_network = String::from(args.ipnetwork);
    match IpNet::from_str(&input_network) {
        Ok(parsed_network) => {
            output_formatter::format(input_network, parsed_network);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
