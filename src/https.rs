//! Module to resolve the HTTPS Connector / Client used to make requests,
//! depending on which TLS implementation we want to use.
//!
//! For instance, when building for the `x86_64-unknown-linux-musl` target
//! like we do for [AWS Lambda][] deployments, we want to prefer to use
//! `hyper_rustls` - which uses a pure Rust implementation of TLS - over the
//! native `hyper_tls` implementation, which uses OpenSSL. For that reason,
//! using the connector from the `hyper_rustls` crate is actually the default.
//!
//! This can be controlled by the optional "features" enabled for this crate:
//!     * `rust-tls`: enables the rust implementation of TLS (default)
//!     * `native-tls`: enables the native implementation of TLS using OpenSSL
//!
//! [AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html
//!
use hyper::client::HttpConnector;
use hyper::Client;

#[cfg(feature = "rust-tls")]
pub(crate) use hyper_rustls as tls;
#[cfg(not(feature = "rust-tls"))]
pub(crate) use hyper_tls as tls;
#[cfg(feature = "rust-tls")]
use rustls::ClientConfig;
#[cfg(feature = "rust-tls")]
use tls::ConfigBuilderExt;

#[cfg(all(feature = "rust-tls", feature = "http2"))]
pub(crate) fn get_https_client() -> Client<tls::HttpsConnector<HttpConnector>> {
    // Prepare the HTTPS connector
    let https_connector = tls::HttpsConnectorBuilder::new()
        // .with_native_roots()
        .with_tls_config(
            ClientConfig::builder()
                .with_safe_defaults()
                .with_native_roots()
                .with_no_client_auth(),
        )
        .https_only()
        .enable_http2()
        .build();

    let mut builder = Client::builder();
    builder.http2_only(true);

    return builder.build::<_, hyper::Body>(https_connector);
}

#[cfg(all(feature = "rust-tls", not(feature = "http2")))]
pub(crate) fn get_https_client() -> Client<tls::HttpsConnector<HttpConnector>> {
    // Prepare the HTTPS connector
    let https_connector = tls::HttpsConnectorBuilder::new()
        // .with_native_roots()
        .with_tls_config(
            ClientConfig::builder()
                .with_safe_defaults()
                .with_native_roots()
                .with_no_client_auth(),
        )
        .https_only()
        .enable_http2()
        .build();

    return Client::builder().build::<_, hyper::Body>(https_connector);
}

#[cfg(not(feature = "rust-tls"))]
pub(crate) fn get_https_client() -> Client<tls::HttpsConnector<HttpConnector>> {
    // Prepare the HTTPS connector
    let https_connector = tls::HttpsConnector::new();
    return Client::builder().build::<_, hyper::Body>(https_connector);
}
