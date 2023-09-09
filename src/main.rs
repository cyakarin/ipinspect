mod ipinspector;

use clap::Parser;
use ipinspector::IpInspector;

#[derive(Parser)]
struct Cli {
    ipnetwork: String
}

fn main() {
    let args = Cli::parse();
    match IpInspector::build(String::from(args.ipnetwork)) {
        Ok(inspector) => {
            inspector.print_for_human();
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
