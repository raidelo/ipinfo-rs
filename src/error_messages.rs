pub const IS_TIMEOUT: &str = "
Possible causes:
  • Server took too long to respond
  • Slow or unstable internet connection
  • Firewall or proxy may be blocking outgoing requests
  • Target server is under heavy load

Troubleshooting:
  1. Make sure your internet connection is stable
  2. Check firewall or proxy settings that might block requests
  3. Wait 1-2 minutes and try again
  4. Verify API status at https://ip-api.com/
  5. If the issue persists, report it to the tool maintainer
";

pub const IS_CONNECT: &str = "
Possible causes:
  • Server is down or unreachable
  • DNS resolution failed
  • No active internet connection
  • Firewall, VPN, or proxy may be blocking the connection

Troubleshooting:
  1. Make sure you have an active internet connection
  2. Check if http://ip-api.com/json is reachable (browser or terminal)
  3. Restart your router or network adapter
  4. Verify DNS settings or try a different DNS server
  5. Review firewall, VPN, or proxy configurations if applicable
  6. If the issue persists, report it to the tool maintainer
";

pub const IS_REQUEST: &str = "
Possible causes:
  • URL or request parameters are invalid or incorrectly formatted

Troubleshooting:
  1. Verify the URL format
  2. If the issue persists, report it to the tool maintainer
";

pub const IS_BUILDER: &str = "
Possible causes:
  • Conflicting or invalid client configuration options
  • Missing required configuration parameters

Troubleshooting:
  1. Double-check your reqwest::Client setup
  2. Remove or fix any conflicting builder options
  3. Make sure all required settings are provided
  4. If the issue persists, report it to the tool maintainer
";

pub const IS_BODY: &str = "
Possible causes:
  • Could not serialize request body data (e.g., JSON serialization failed)
  • Body type is not supported by reqwest
  • Headers are incorrect or do not match the body format (e.g., Content-Type does not match)

Troubleshooting:
  1. Make sure the request body is valid (e.g., JSON string, UTF-8 text, or bytes)
  2. Verify that the Content-Type header matches the body format
  3. Avoid setting Content-Length manually — let reqwest handle it
  4. If the issue persists, report it to the tool maintainer
";

pub const IS_REDIRECT: &str = "
Possible causes:
  • Server is causing a redirect loop
  • Request exceeded the maximum number of allowed redirects

Troubleshooting:
  1. Adjust the maximum number of allowed redirects in your client
  2. Manually check redirect URLs to identify loops
  3. If the issue persists, report it to the tool maintainer
";

pub const IS_DECODE: &str = "
Possible causes:
  • API response format changed or is not as expected
  • Data might be corrupted during transmission

Troubleshooting:
  1. Verify API status at https://ip-api.com/
  2. Update this tool to the latest version
  3. If the issue persists, report it to the tool maintainer
";

pub const API_RATE_LIMIT_EXEEDED: &str = "
Possible causes:
  • API limit of 45 requests per minute exceeded
  • IP address may be shared with other users

Troubleshooting:
  1. Wait 1-2 minutes and try again
  2. Consider using a different IP address if possible
  3. Verify API status at https://ip-api.com/
  4. If the issue persists, report it to the tool maintainer
";

pub const CLIENT_ERROR: &str = "
Possible causes:
  • Invalid or missing request parameters, or unsupported input format

Troubleshooting:
  1. Verify your input values
  2. Check the API documentation for allowed formats and constraints
  3. If the issue persists, report it to the tool maintainer
";

pub const SERVER_ERROR: &str = "
Possible causes:
  • API service is down, overloaded, or undergoing maintenance

Troubleshooting:
  1. Wait 1-2 minutes and try again
  2. Verify API status at https://ip-api.com/
  3. If the issue persists, report it to the tool maintainer
";

pub const UNKNOWN_HTTP_ERROR: &str = "
Possible causes:
  • Unexpected or unknown HTTP response
  • Network issues or connectivity problems
  • Rare or unsupported status code

Troubleshooting:
  1. Check your internet connection
  2. Run the command again
  3. Update the tool to the latest version
  4. If the issue persists, report it to the tool maintainer
";

pub const UNKNOWN_NETWORK_ERROR: &str = "
error: 🌀 Unknown network error

Possible causes:
  • Unexpected or unknown network or system error

Troubleshooting:
  1. Verify your network settings and connections
  2. Make sure your internet is working properly
  3. If the issue persists, report it to the tool maintainer
";
