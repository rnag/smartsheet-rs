//! Utilities to validate a response to ensure that its *status code*
//! indicates that it is a success.
//!
use crate::log::error;
use crate::models::{RequestError, SmartsheetError};
use crate::types::Result;
use crate::utils::resp_to_string;

use core::option::Option::Some;
use core::result::Result::{Err, Ok};

use http::Response;
use hyper::{http, Body};

/// Check the `status` of a Response and raise an error (RequestError) if the
/// request was not a success.
///
/// Adapted from the Python `requests` library, specifically the
/// `requests.models.raise_for_status` function - link is below.
///
/// This function checks if the status code of the response is between
/// 400 and 600 to see if there was a client error or a server error. If
/// the status code, is between 200 and 400, this will return `Ok()`. This
/// is **not** a check to see if the response code is ``200 OK``.
///
/// # Credits
///
/// https://github.com/psf/requests/blob/95f456733656ed93645ff0250bfa54f6d256f6fe/requests/models.py#L945
///
pub async fn raise_for_status(request_url: String, resp: &mut Response<Body>) -> Result<()> {
    let status_code = resp.status().as_u16();
    let reason: String;

    if (400..500).contains(&status_code) {
        // Client Error
        reason = format!(
            "{status} Client Error: {reason} for url: {url}",
            status = status_code,
            reason = resp.status().canonical_reason().unwrap_or("Unknown"),
            url = request_url,
        );
    } else if (500..600).contains(&status_code) {
        // Server Error
        reason = format!(
            "{status} Server Error: {reason} for url: {url}",
            status = status_code,
            reason = resp.status().canonical_reason().unwrap_or("Unknown"),
            url = request_url,
        );
    } else {
        // Not an error at least, so we're safe.
        return Ok(());
    }

    let resp_data = resp_to_string(resp).await?;

    let mut e = RequestError::new(status_code, reason);

    // Attempt to de-serialize the response data into a `SmartsheetError`
    // object, and set the `error` field. If there are any errors in
    // de-serializing the data, we populate the `message` field instead.
    if let Ok(error_data) = serde_json::from_str::<SmartsheetError>(&resp_data) {
        e.error = Some(error_data);
    } else {
        e.message = Some(resp_data.trim().to_string());
    }

    error!("{:#?}", e);

    Err(Box::new(e))
}
