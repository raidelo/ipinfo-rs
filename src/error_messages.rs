pub const IS_TIMEOUT: &str = "
Possible causes:
  • Server took too long to respond
  • Unstable or slow internet/network connection
  • Firewall or proxy blocking outgoing requests

Troubleshooting:
  1. Ensure your internet connection is stable
  2. Check firewall or proxy settings that might block the request
  3. Wait 1-2 minutes and try again
  4. Verify API status at https://ip-api.com/
  5. Report this issue to the tool maintainer if the problem persists
";

pub const IS_CONNECT: &str = "
Possible causes:
  • The server might be down or unreachable
  • DNS resolution failure (cannot translate domain name)
  • No active internet connection

Troubleshooting:
  1. Ensure you have an active internet connection
  2. Open https://ip-api.com in your web browser to verify availability
  3. Use 'curl http://ip-api.com/json' to test connectivity from your terminal
  4. Restart your router or network adapter
  5. Check your DNS settings and try a different DNS server if necessary
  6. Report this issue to the tool maintainer if the problem persists
";

pub const IS_REQUEST: &str = "
Possible causes:
  • URL format is incorrect or contains invalid characters
  • HTTP method is not supported by the API
  • Request headers have invalid or missing values

Troubleshooting:
  1. Check that the URL is correctly formatted
  2. Confirm you are using a valid HTTP method (GET, POST, etc.)
  3. Review your request headers for correctness
  4. Report this issue to the tool maintainer if the problem persists
";

pub const IS_BUILDER: &str = "
Possible causes:
  • Client is misconfigured or has conflicting options
  • Settings conflict with each other causing build failure
  • Missing required client configuration settings

Troubleshooting:
  1. Review your reqwest::Client configuration code
  2. Remove or correct conflicting builder options
  3. Ensure all required settings are provided
  4. Report this issue to the tool maintainer if the problem persists
";

pub const IS_BODY: &str = "
Possible causes:
  • Failed to convert data into request body format
  • Provided body type is not supported by reqwest
  • Missing or incorrect Content-Length header

Troubleshooting:
  1. Ensure the request body data is properly formatted (UTF-8 or bytes)
  2. Verify that Content-Type header matches the body format
  3. Check if Content-Length header is set correctly or omit it to let reqwest handle it
  4. Report this issue to the tool maintainer if the problem persists
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
