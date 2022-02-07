/// Library-specific constants

/// Base API endpoint for Smartsheet
pub const API_ENDPOINT: &str = "https://api.smartsheet.com/2.0";

/// Environment variable to be used to retrieve the API token,
/// when `SmartsheetApi::fromenv()` is invoked.
///
/// Note that this is the same variable name that the official
/// SDK uses.
///
/// For example, refer to the Python SDK:
///    https://github.com/smartsheet-platform/smartsheet-python-sdk#getting-started
///
pub const ENV_VAR_NAME: &str = "SMARTSHEET_ACCESS_TOKEN";
