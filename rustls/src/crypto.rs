#[cfg(feature = "wasi_crypto")]
mod wasi_crypto_impl;

mod aead;
mod agreement;
pub(crate) mod constant_time;
mod digest;
mod hkdf;
mod hmac;
mod rand;
mod signature;

// mod io::der::Tag::Sequence; TODO: need investigation

#[test]
fn test_constant_time() {
    match constant_time::verify_slices_are_equal(&[1, 2], &[1, 2]) {
        Ok(_) => {}
        Err(e) => {
            dbg!(e);
        }
    }
}

// 列出清单
