mod commands;
mod system_checks;

use clap::{Arg, ArgGroup, Command};
use std::process;

use crate::commands::{dns, latency, throughput, traceroute};
use crate::system_checks::check_traceroute_availability;

#[tokio::main]
async fn main() {
    let matches = Command::new("NetPerf CLI")
        .version("0.1.0")
        .author("Mitch Vostrez <mitch@voz.dev>")
        .about("CLI tool for network performance monitoring")
        // Network Testing Commands
        .arg(
            Arg::new("latency")
                .short('l')
                .long("latency")
                .help("Check network latency using TCP")
                .long_help("Performs TCP connection tests to Google DNS (8.8.8.8) and calculates average latency")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Number of packets/requests to send")
                .value_name("COUNT")
                .value_parser(clap::value_parser!(u32))
                .default_value("5"),
        )
        .arg(
            Arg::new("throughput")
                .short('t')
                .long("throughput")
                .help("Measure download speed using a test file")
                .long_help("Downloads a 100MB test file from Hetzner's speed test server to measure throughput")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dns")
                .short('d')
                .long("dns")
                .help("Test DNS resolution time for a domain")
                .long_help("Measures the time taken to resolve a domain name using your system's DNS resolver")
                .value_name("DOMAIN")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("traceroute")
                .short('r')
                .long("traceroute")
                .help("Perform a traceroute to a host")
                .long_help("Traces the network path to a specified host, showing all intermediate hops")
                .value_name("HOST")
                .value_parser(clap::value_parser!(String)),
        )
        // Output Options
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Specify output format")
                .long_help("Set the output format for test results. Supported formats: plain, json")
                .value_name("FORMAT")
                .value_parser(["plain", "json"]),
        )
        // Group the main commands
        .group(
            ArgGroup::new("commands")
                .args(["latency", "throughput", "dns", "traceroute"])
                .required(false)
                .multiple(true),
        )
        .after_help(
            "EXAMPLES:\n\
            netperf --latency                        # Check network latency\n\
            netperf --throughput                     # Measure download speed\n\
            netperf --dns example.com                # Test DNS resolution\n\
            netperf --traceroute 8.8.8.8             # Perform traceroute\n\
            netperf -l -t                            # Run both latency and throughput tests\n\
            netperf --dns example.com --output json  # DNS test with JSON output"
        )
        .get_matches();

    if matches.get_flag("latency") {
        let count = matches.get_one::<u32>("count").copied().unwrap_or(5);
        if let Err(err) = latency::check(count).await {
            eprintln!("Error checking latency: {}", err);
            process::exit(1);
        }
    }

    if matches.get_flag("throughput") {
        if let Err(err) = throughput::check().await {
            eprintln!("Error measuring throughput: {}", err);
            process::exit(1);
        }
    }

    if let Some(domain) = matches.get_one::<String>("dns") {
        if let Err(err) = dns::check(domain).await {
            eprintln!("Error checking DNS: {}", err);
            process::exit(1);
        }
    }

    if let Some(host) = matches.get_one::<String>("traceroute") {
        if let Err(err) = traceroute::perform(host).await {
            eprintln!("Error performing traceroute: {}", err);
            process::exit(1);
        }
    }

    if matches.get_one::<String>("traceroute").is_some() && !check_traceroute_availability() {
        eprintln!("Warning: Traceroute command not found. Please install:");
        if cfg!(windows) {
            eprintln!("  - Windows: tracert is part of the base system");
        } else if cfg!(target_os = "linux") {
            eprintln!("  - Linux: sudo apt-get install inetutils-traceroute");
            eprintln!("    or: sudo apt-get install iputils-tracepath");
        } else if cfg!(target_os = "macos") {
            eprintln!("  - macOS: traceroute is part of the base system");
        }
    }

    println!("Network performance check completed.");
}
