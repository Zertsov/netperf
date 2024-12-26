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

In the root of the repo, run `cargo build`. This will add a built version to `target`.