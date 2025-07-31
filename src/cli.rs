use core::net::IpAddr;

use clap::{Arg, ArgMatches, Command};

fn ip_parser(arg: &str) -> Result<IpAddr, String> {
    arg.parse()
        .map_err(|_| format!("'{}' is not a valid IPv4/IPv6 address", arg))
}

pub fn parse_args() -> ArgMatches {
    const STYLES: [&str; 5] = ["thin", "rounded", "double", "thick", "basic"];

    Command::new("ipinfo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Raidel <raidelosix@gmail.com>")
        .about("Retrieves detailed information about IP addresses")
        .arg(
            Arg::new("ip")
                .required(false)
                .value_name("ADDRESS")
                .value_parser(ip_parser)
                .help("Target IPv4/IPv6 address [default: your public IP]"),
        )
        .arg(
            Arg::new("style")
                .short('s')
                .long("style")
                .value_name("STYLE")
                .value_parser(STYLES)
                .default_value("rounded")
                .hide_possible_values(true)
                .hide_default_value(true)
                .help(format!(
                    "Visual style for the output table border
  [possible values: {}]
  [default: rounded]",
                    STYLES.join(", ")
                ))
                .long_help(
                    "Defines the border style for the displayed information table.

Available options:
  - thin: Clean thin lines
  - rounded: Modern rounded corners
  - double: Classic double-line border
  - thick: Bold thick lines
  - basic: Simple ASCII characters

[default: rounded]",
                ),
        )
        .get_matches_mut()
}
