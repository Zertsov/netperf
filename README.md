# NetPerf CLI

A command-line network performance testing tool written in Rust. NetPerf CLI provides various network diagnostics including latency checks, throughput measurements, DNS resolution testing, and traceroute functionality.

## Features

- **Latency Testing**: TCP-based latency measurements to a target server
- **Throughput Testing**: Download speed measurements using various test servers
- **DNS Resolution**: DNS lookup performance testing with detailed metrics
- **Traceroute**: Network path analysis (platform-specific implementation)

## Installation

### Prerequisites
- Rust toolchain (1.70 or later)
- For traceroute functionality:
  - Windows: `tracert` (included by default)
  - Linux: `tracepath` or `traceroute` (`sudo apt-get install inetutils-traceroute`)
  - macOS: `traceroute` (included by default)

### Building from source 

```bash
git clone https://github.com/zertsov/netperf
cd netperf
cargo build --release
```

The binary will be available at `target/release/netperf`

## Usage

### Basic latency test
```bash
netperf --latency
```
### Throughput (speed) test
```bash
netperf --throughput
```
### DNS resolution test
```bash
netperf --dns example.com
```
### Traceroute to a host
```bash
netperf --traceroute 8.8.8.8
```
### Multiple tests at once
```bash
netperf -l -t # Run both latency and throughput tests
```
### Specify number of latency test packets
```bash
netperf --latency --count 10
```

### CLI Options

OPTIONS:

`-l, --latency` Check network latency using TCP

`-t, --throughput` Measure download speed using a test file

`-d, --dns <DOMAIN>` Test DNS resolution time for a domain

`-r, --traceroute <HOST>` Perform a traceroute to a host

`-c, --count <COUNT>` Number of packets/requests to send [default: 5]

`-o, --output <FORMAT>` Specify output format (plain, json)

`-h, --help` Print help information

`-V, --version` Print version information

## Output Formats

The tool supports two output formats:
- `plain`: Human-readable text output (default)
- `json`: JSON formatted output (useful for programmatic processing)

## Examples

### Basic latency test

```bash
$ netperf --latency
Testing latency to 8.8.8.8:53 for 5 attempts...
Response time: 15 ms
Response time: 14 ms
Response time: 16 ms
Response time: 14 ms
Response time: 15 ms
Average latency: 14.80 ms (5 successful attempts)
```

### DNS resolution test

```bash
$ netperf --dns google.com
Testing DNS resolution for google.com...
DNS lookup attempt 1 of 3...
Resolved in 45 ms. Found addresses:
142.250.190.78
2607:f8b0:4004:c09::67
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the no license.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [hickory-resolver](https://crates.io/crates/hickory-resolver) for DNS resolution
- Uses [tokio](https://tokio.rs/) for async runtime
- Uses [clap](https://clap.rs/) for CLI argument parsing