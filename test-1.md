# ipinfo-rs

Fast, lightweight CLI tool to fetch and display geolocation data for any IPv4 address.

## Features

- Zero-config: shows your **public IP** if no address is provided.  
- 5 built-in table styles: `thin`, `rounded`, `double`, `thick`, `basic`.  
- Colorful, human-readable error messages.  
- Tiny single binary (~1 MB, no runtime dependencies).  
- 100 % safe Rust.

## Installation

### Cargo
```bash
cargo install ipinfo-rs
```

### Pre-built binaries
Grab the latest release for your platform from  
[Releases](https://github.com/your-user/ipinfo-rs/releases).

## Usage

```bash
# Show info for your own public IP
ipinfo-rs

# Show info for a specific IP
ipinfo-rs 8.8.8.8

# Choose a different table style
ipinfo-rs 1.1.1.1 --style thick
```

### Options

| Flag | Description |
|------|-------------|
| `[ADDRESS]` | Target IPv4 address (optional) |
| `-s, --style <STYLE>` | Table border style (default: `rounded`) |

Available styles: `thin`, `rounded`, `double`, `thick`, `basic`.

## Examples

```text
$ ipinfo-rs 8.8.8.8 --style rounded
╭────────────┬──────────────╮
│    query   │ 8.8.8.8      │
├────────────┼──────────────┤
│  country   │ United States│
│  city      │ Mountain View│
│   isp      │ Google LLC   │
╰────────────┴──────────────╯
```

## Build from source

```bash
git clone https://github.com/your-user/ipinfo-rs.git
cd ipinfo-rs
cargo build --release
# Binary will be at target/release/ipinfo-rs
```

## Tests

Unit tests:

```bash
cargo test
```

Integration tests (spins up a local mock server):

```bash
cargo test --test integration_test
```

## API limits

This tool queries **ip-api.com**’s free endpoint, limited to **45 requests/minute**.  
If you need higher limits or SSL, check [ip-api.com Pro](https://ip-api.com/docs/#pro).

## License

MIT © Raidel
