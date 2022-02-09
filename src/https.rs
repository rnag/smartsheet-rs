///! Module to resolve the HTTPS Connector used to make requests, depending on
///! which TLS implementation we want to use.
///!
///! For instance, when building for the `x86_64-unknown-linux-musl` target
///! like we do for [AWS Lambda][] deployments, we want to prefer the
///! `hyper_rustls` over the native `hyper_tls` implementation, which uses
///! OpenSSL. For that reason, using the connector from the `hyper_rustls`
///! crate is actually the default.
///!
///! This can be controlled by the optional "features" enabled for this crate:
///!     * `rust-tls`: enables the rust implementation of TLS (default)
///!     * `native-tls`: enables the native implementation using OpenSSL
///!
///! [AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html
///!
use hyper::client::HttpConnector;

#[cfg(feature = "rust-tls")]
pub(crate) use hyper_rustls as tls;
#[cfg(feature = "native-tls")]
pub(crate) use hyper_tls as tls;
#[cfg(feature = "rust-tls")]
use rustls;
#[cfg(feature = "rust-tls")]
use tls::ConfigBuilderExt;

#[cfg(feature = "rust-tls")]
pub(crate) fn get_connector() -> tls::HttpsConnector<HttpConnector> {
    let tls2 = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_native_roots()
        .with_no_client_auth();

    // Prepare the HTTPS connector
    tls::HttpsConnectorBuilder::new()
        // .with_native_roots()
        .with_tls_config(tls2)
        .https_only()
        // .https_or_http()
        .enable_http2()
        // .enable_http1()
        .build()
}

#[cfg(feature = "native-tls")]
pub(crate) fn get_connector() -> tls::HttpsConnector<HttpConnector> {
    tls::HttpsConnector::new()
}
