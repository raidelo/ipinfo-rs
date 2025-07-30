# ipinfo-rs - IP Information Tool ![Rust](https://img.shields.io/badge/language-Rust-orange.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> A command-line tool to gather detailed information about IP addresses with beautiful table formatting

## Features ✨

- 🌐 Get detailed IP geolocation information
- 📊 Display results in customizable table formats
- ⚡ Fast and efficient with async requests
- 🎨 Multiple table border styles:
  - Thin (`-s thin`)
  - Rounded (`-s rounded`)
  - Double lines (`-s double`)
  - Thick (`-s thick`)
  - Basic ASCII (`-s basic`)
- 🛡️ Robust error handling with helpful messages

## Installation 📦

### From Source (requires Rust toolchain)
```bash
cargo install ipinfo-rs
```

### Pre-compiled Binaries
Download from [Releases page](https://github.com/yourusername/ipinfo-rs/releases)

## Usage 🚀

```bash
ipinfo-rs [OPTIONS] [ADDRESS]
```

### Arguments
| Argument    | Description                               | Default     |
|-------------|-------------------------------------------|-------------|
| `[ADDRESS]` | Target IPv4 address                       | Your own IP |

### Options
| Flag          | Description                               | Values | Default     |
|---------------|-------------------------------------------|--------|-------------|
| `-s, --style` | Visual style for the output table border  | `thin`, `rounded`, `double`, `thick`, `basic` | `rounded` |
| `-h, --help`  | Print help (see detailed help with `--help`) | | |
| `-V, --version` | Print version | | |

### Examples
```bash
# Get info about your own IP
ipinfo-rs

# Get info about Google DNS with double-line table
ipinfo-rs 8.8.8.8 --style double

# Use basic ASCII style
ipinfo-rs -s basic
```

## Sample Output 📄
```text
+-------------------------------------+--------------------------------+
|               Property               |             Value              |
+-------------------------------------+--------------------------------+
| IP Address                          | 8.8.8.8                        |
| Country                             | United States                  |
| Region                              | California                     |
| City                                | Mountain View                  |
| ISP                                 | Google LLC                     |
| Latitude/Longitude                  | 37.4056, -122.0775             |
| Timezone                            | America/Los_Angeles            |
+-------------------------------------+--------------------------------+
```

## Detailed Help 🆘
```bash
ipinfo-rs --help
```

Output:
```text
Retrieves detailed information about IP addresses

Usage: ipinfo-rs [OPTIONS] [ADDRESS]

Arguments:
  [ADDRESS]  Target IPv4 address [default: your public IP]

Options:
  -s, --style <STYLE>  Visual style for the output table border
                         [possible values: thin, rounded, double, thick, basic]
                         [default: rounded]
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

## Error Handling 🛠️
The tool provides clear error messages with troubleshooting suggestions:

```text
error: ⌛ Timeout after 5000ms
hint: Check your internet connection or firewall settings

error: 🔍 Client error (404): Not Found
hint: The requested resource was not found
```

## Development 🧪

### Build
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Contributing
Contributions are welcome! Please open an issue or pull request on GitHub.

## License 📜
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**ipinfo-rs** - Because IP information should be beautiful and accessible! ✨
