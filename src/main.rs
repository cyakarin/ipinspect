mod output_formatter;
mod network_parser;

use clap::Parser;
use network_parser::NetworkParser;

#[derive(Parser)]
struct Cli {
    ipnetwork: String
}

fn main() {
    let args = Cli::parse();
    let input_network = String::from(args.ipnetwork);
    let parser = NetworkParser::new(String::from(&input_network));
    match parser.parse() {
        Ok(parsed_network) => {
            output_formatter::format(input_network, parsed_network);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
