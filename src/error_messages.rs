pub const IS_TIMEOUT: &str = "
Possible causes:
  • The server took too long to respond
  • Slow or unstable internet connection
  • Firewall or proxy blocking outgoing requests
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
  • The server is down or unreachable
  • DNS resolution failure (domain name cannot be resolved)
  • No active internet connection
  • Firewall, VPN, or proxy blocking the connection

Troubleshooting:
  1. Make sure you have an active internet connection
  2. Open https://ip-api.com in your web browser to verify availability
  3. Use 'curl http://ip-api.com/json' to test connectivity from your terminal
  4. Restart your router or network adapter
  5. Check your DNS settings or try a different DNS server
  6. If behind a firewall, VPN, or proxy, review their configurations
  7. If the issue persists, report it to the tool maintainer
";

pub const IS_REQUEST: &str = "
Possible causes:
  • URL format is incorrect or contains invalid characters
  • HTTP method is not supported by the API
  • Request headers have invalid or missing values
  • Request parameters are missing or incorrectly formatted

Troubleshooting:
  1. Check that the URL is correctly formatted
  2. Confirm you are using a valid HTTP method (GET, POST, etc.)
  3. Review your request headers for correctness
  4. Review request parameters for correctness
  5. If the issue persists, report it to the tool maintainer
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
  • Server is redirecting requests in a loop
  • Exceeded the maximum allowed redirects

Troubleshooting:
  1. Set a reasonable limit for redirects in client settings
  2. Manually check redirect URLs to identify loops
  3. Report this issue to the tool maintainer if the problem persists
";

pub const IS_DECODE: &str = "
Possible causes:
  • API response format has changed or is unexpected
  • Server returned data not matching expected structure
  • Possible data corruption during transmission

Troubleshooting:
  1. Verify API status at https://ip-api.com/
  2. Update this tool to the latest version
  3. Report this issue to the tool maintainer with details
";

pub const API_RATE_LIMIT_EXEEDED: &str = "
Possible causes:
  • Rate limit of 45 requests per minute exceeded
  • API key or IP address shared among multiple users

Troubleshooting:
  1. Wait 1-2 minutes and try again
  2. Consider using a different IP or API key
  3. Verify API status at https://ip-api.com/
  4. Report this issue to the tool maintainer if the problem persists
";

pub const CLIENT_ERROR: &str = "
Possible causes:
  • Request parameters are invalid or missing
  • Input format not supported by the API

Troubleshooting:
  1. Verify your input values
  2. Consult the API documentation for allowed formats and constraints
  3. Report the issue to the tool maintainer if it persists
";

pub const SERVER_ERROR: &str = "
Possible causes:
  • API service is down or overloaded
  • Scheduled maintenance in progress
  • Temporary server issues

Troubleshooting:
  1. Wait 1-2 minutes and try again
  2. Check API status at https://ip-api.com/
  3. Contact the API provider if the problem persists
  4. Report this issue to the tool maintainer if the problem persists
";

pub const UNKNOWN_HTTP_ERROR: &str = "";

pub const UNKNOWN_NETWORK_ERROR: &str = "
Possible causes:
  • Possible misconfiguration of your network or system
  • Unexpected or unknown network error occurred

Troubleshooting:
  1. Verify your network settings and connections
  2. Ensure your internet is working properly
  3. Contact support if the problem continues
  4. Report this issue to the tool maintainer if the problem persists
";
