use std::net::SocketAddr;
use std::str::FromStr;

use crate::dns_types::RecordType;
use ascii::AsciiString;
use clap;
use clap::Parser;

/// DNS testing connection to dns server not using local resolver
#[derive(Parser, Debug)]
#[command(author="Pieter E Smit", version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CmdArgs {
    /// Choose the DNS record type (supports A, CNAME, SOA and AAAA)
    #[clap(long, default_value = "A")]
    pub record_type: RecordType,

    /// specifie seconds interval to repeat dns queries.
    #[clap(short, long, default_value = "0")]
    pub interval: u64,

    /// Which DNS resolver to query
    #[clap(short, long, default_value = "1.1.1.1:53", value_parser = parser_socket_addr_and_port )]
    pub resolver: SocketAddr,

    /// A domain name to look up. Remember, these must be ASCII.
    #[clap(default_value = "google.com.", value_parser = parser_abs_dns_name)]
    //forbid_empty_values = true)]
    pub name: String,

    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// specify msec seen as to high.
    //#[clap(short, long, value_parser, num_args = 0.., value_delimiter = ',')]
    #[clap(short, long, num_args = 0..=3, value_delimiter = ',', default_value = "1000")]
    pub slow: Vec<u64>,
}

fn parser_abs_dns_name(s: &str) -> Result<String, String> {
    let mut name = s.to_string();
    if AsciiString::from_str(&name).is_err() {
        eprintln!("DNS names must be ASCII, and {name} is not.");
    }
    if !name.ends_with('.') {
        name.push('.');
    }
    Ok(name)
}
fn parser_socket_addr_and_port(s: &str) -> Result<SocketAddr, String> {
    let mut addr = s.to_string();
    if !addr.contains(':') {
        addr.push_str(":53");
    }
    match addr.parse::<SocketAddr>() {
        Ok(addr) => Ok(addr),
        Err(e) => Err(format!("Error parsing socket address: {}", e)),
    }
}
