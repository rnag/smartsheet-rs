//! Library-specific utilities, mainly for internal use.
//!
use crate::types::Result;

use std::io::{BufReader, Read};

use hyper::body::Buf;
use hyper::{Body, Response};
use serde::de;

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

#[cfg(feature = "serde-std")]
pub async fn resp_into_struct<T>(resp: Response<Body>) -> Result<T>
where
    T: de::DeserializeOwned,
{
    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(resp).await?;

    // use a buffered reader
    let reader = BufReader::new(body.reader());

    // try to parse as json with serde_json
    Ok(serde_json::from_reader(reader)?)
}

pub async fn into_struct_from_str<T>(resp: Response<Body>) -> Result<T>
where
    T: de::DeserializeOwned,
{
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

    Ok(serde_json::from_str(std::str::from_utf8(&body_bytes)?)?)
}

pub async fn into_struct_from_slice<T>(resp: Response<Body>) -> Result<T>
where
    T: de::DeserializeOwned,
{
    // asynchronously concatenate the buffer from a body into bytes
    let bytes = hyper::body::to_bytes(resp).await?;

    // try to parse as json with serde_json
    Ok(serde_json::from_slice(&bytes)?)
}

/// Read the body content of a mutable reference to a `Response` object
/// into a string.
pub async fn resp_to_string(resp: &mut Response<Body>) -> Result<String> {
    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(resp).await?;

    // use a buffered reader
    let mut reader = BufReader::new(body.reader());

    // read BufReader contents into a string
    let mut body_string = String::new();
    reader.read_to_string(&mut body_string)?;

    Ok(body_string)
}
