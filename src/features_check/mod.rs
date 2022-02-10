//! Shows a user-friendly compiler error on incompatible selected features.
//!
//! Credits: https://github.com/serde-rs/json/blob/master/src/features_check/mod.rs

#[allow(unused_macros)]
macro_rules! hide_from_rustfmt {
    ($mod:item) => {
        $mod
    };
}

#[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
hide_from_rustfmt! {
    mod error;
}
