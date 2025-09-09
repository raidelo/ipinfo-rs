pub const IS_TIMEOUT: &str = "\
\u{231b} Connection timed out (%(timeout)ss)

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Server took too long to respond
  \u{2022} Slow or unstable internet connection
  \u{2022} Firewall or proxy may be blocking outgoing requests
  \u{2022} Target server is under heavy load

\x1b[1mTroubleshooting:\x1b[0m
  1. Make sure your internet connection is stable
  2. Check firewall or proxy settings that might block requests
  3. Wait a few minutes and try again
  4. Check API status at https://ip-api.com/
  5. Report the issue to the tool maintainer if it persists";

pub const IS_CONNECT: &str = "\
\u{01f50c} Connection failed

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Server is down or unreachable
  \u{2022} DNS resolution failed
  \u{2022} No active internet connection
  \u{2022} Firewall, VPN, or proxy may be blocking the connection

\x1b[1mTroubleshooting:\x1b[0m
  1. Make sure you have an active internet connection
  2. Test if http://ip-api.com/json is reachable (browser or terminal)
  3. Restart your router or network adapter
  4. Verify DNS settings or try a different DNS server
  5. Review firewall, VPN, or proxy configurations if applicable
  6. Report the issue to the tool maintainer if it persists";

pub const IS_REQUEST: &str = "\
\u{01f4dc} Invalid request

\x1b[1mPossible causes:\x1b[0m
  \u{2022} URL or request parameters are invalid or incorrectly formatted

\x1b[1mTroubleshooting:\x1b[0m
  1. Verify the URL format
  2. Update the tool to the latest version
  3. Report the issue to the tool maintainer if it persists";

pub const IS_BUILDER: &str = "\
\u{01f6e0} Request builder error

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Conflicting or invalid client configuration options
  \u{2022} Missing required configuration parameters

\x1b[1mTroubleshooting:\x1b[0m
  1. Double-check your reqwest::Client setup
  2. Remove or fix any conflicting builder options
  3. Make sure all required settings are provided
  4. Update the tool to the latest version
  5. Report the issue to the tool maintainer if it persists";

pub const IS_BODY: &str = "\
\u{01f4e6} Invalid response body

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Could not serialize request body data (e.g., JSON serialization failed)
  \u{2022} Body type is not supported by reqwest
  \u{2022} Headers are incorrect or do not match the body format (e.g., Content-Type does not match)

\x1b[1mTroubleshooting:\x1b[0m
  1. Make sure the request body is valid (e.g., JSON string, UTF-8 text, or bytes)
  2. Verify that the Content-Type header matches the body format
  3. Avoid setting Content-Length manually — let reqwest handle it
  4. Update the tool to the latest version
  5. Report the issue to the tool maintainer if it persists";

pub const IS_REDIRECT: &str = "\
\u{01f504} Too many redirects

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Server is causing a redirect loop
  \u{2022} Request exceeded the maximum number of allowed redirects

\x1b[1mTroubleshooting:\x1b[0m
  1. Adjust the maximum number of allowed redirects in your client
  2. Manually check redirect URLs to identify loops
  3. Report the issue to the tool maintainer if it persists";

pub const IS_DECODE: &str = "\
\u{01f4c4} Response decoding error

\x1b[1mPossible causes:\x1b[0m
  \u{2022} API response format changed or is not as expected
  \u{2022} Data might be corrupted during transmission

\x1b[1mTroubleshooting:\x1b[0m
  1. Check API status at https://ip-api.com/
  2. Update the tool to the latest version
  3. Report the issue to the tool maintainer if it persists";

pub const API_RATE_LIMIT_EXEEDED: &str = "\
\u{01f40c} API rate limit exceeded

\x1b[1mPossible causes:\x1b[0m
  \u{2022} API limit of 45 requests per minute exceeded
  \u{2022} IP address may be shared with other users

\x1b[1mTroubleshooting:\x1b[0m
  1. Wait a few minutes and try again
  2. Consider using a different IP address if possible
  3. Check API status at https://ip-api.com/
  4. Report the issue to the tool maintainer if it persists";

pub const CLIENT_ERROR: &str = "\
\u{01f464} Client error (%(status_code)d)%(reason)s

\x1b[1mPossible causes:\x1b[0m
  \u{2022} API endpoint or access policy may have changed

\x1b[1mTroubleshooting:\x1b[0m
  1. Wait a few minutes and try again
  2. Check API status at https://ip-api.com/
  3. Update the tool to the latest version
  4. Report the issue to the tool maintainer if it persists";

pub const SERVER_ERROR: &str = "\
\u{01f310} Server error (%(status_code)d)%(reason)s

\x1b[1mPossible causes:\x1b[0m
  \u{2022} API service is down, overloaded, or undergoing maintenance

\x1b[1mTroubleshooting:\x1b[0m
  1. Wait a few minutes and try again
  2. Check API status at https://ip-api.com/
  3. Update the tool to the latest version
  4. Report the issue to the tool maintainer if it persists";

pub const UNKNOWN_HTTP_ERROR: &str = "\
\u{26a0}\u{fe0f} HTTP error (%(status_code)d)%(reason)s

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Unexpected HTTP status code (unsupported)
  \u{2022} Temporary server issue

\x1b[1mTroubleshooting:\x1b[0m
  1. Run the command again
  2. Check API status at https://ip-api.com/
  3. Update the tool to the latest version
  4. Report the issue to the tool maintainer if it persists";

pub const UNKNOWN_NETWORK_ERROR: &str = "\
\u{01f300} Unknown network error

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Unexpected or unknown network or system error

\x1b[1mTroubleshooting:\x1b[0m
  1. Verify your network settings and connections
  2. Make sure your internet is working properly
  3. Report the issue to the tool maintainer if it persists";

pub const BODY_READING_ERROR: &str = "\
\u{01f4c4} Failed to read response body

\x1b[1mPossible causes:\x1b[0m
  \u{2022} Response data is not valid UTF-8
  \u{2022} Server returned malformed or incomplete data
  \u{2022} Unexpected error while reading the response

\x1b[1mTroubleshooting:\x1b[0m
  1. Make sure your internet connection is working properly
  2. Test if http://ip-api.com/json is reachable (browser or terminal)
  3. Check API status at https://ip-api.com/
  4. Update the tool to the latest version
  5. Report the issue to the tool maintainer if it persists";

pub const EMPTY_RESPONSE_FROM_SERVER: &str = "\
\u{01f4ed} Empty API response

\x1b[1mPossible causes:\x1b[0m
  \u{2022} API service is down, overloaded, or undergoing maintenance
  \u{2022} Unexpected server error resulted in no response body

\x1b[1mTroubleshooting:\x1b[0m
  1. Check API status at https://ip-api.com/
  2. Wait a few minutes and try again
  3. Report the issue to the tool maintainer if it persists";

pub const API_RESPONSE_JSON_PARSING_ERROR: &str = "\
\u{01f4c4} Failed to parse API response

\x1b[1mDetail:\x1b[0m %(detail)s

\x1b[1mPossible causes:\x1b[0m
  \u{2022} The API service may have changed its response format
  \u{2022} Response was not valid JSON (e.g., an ISP login page or HTML error)

\x1b[1mTroubleshooting:\x1b[0m
  1. Check API status at https://ip-api.com/
  2. Make sure your internet connection is not behind a login page
  3. Update the tool to the latest version
  4. Report the issue to the tool maintainer if it persists";

pub const INVALID_API_RESPONSE_TYPE: &str = "\
\u{01f9e9} Invalid API response type

\x1b[1mExpected:\x1b[0m JSON object
\x1b[1mReceived:\x1b[0m %(received)s

\x1b[1mPossible causes:\x1b[0m
  \u{2022} The API service may have changed its response format

\x1b[1mTroubleshooting:\x1b[0m
  1. Check API status at https://ip-api.com/
  2. Update the tool to the latest version
  3. Report the issue to the tool maintainer if it persists";
