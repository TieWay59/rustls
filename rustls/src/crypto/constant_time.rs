#[cfg(feature = "ring_webpki")]
pub(crate) use ring::constant_time::verify_slices_are_equal;

#[cfg(feature = "wasi_crypto")]
pub(crate) use super::wasi_crypto_impl::constant_time::verify_slices_are_equal;
