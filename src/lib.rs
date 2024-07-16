use std::ffi;
use std::panic;
use std::ptr;
use std::sync::Arc;

/// Returns a pointer that may be cast to a rustls-ffi `*rustls_crypto_provider`.
/// You must use `rustls_crypto_provider_free` to free the pointer when done.
///
/// The returned provider is equivalent to `rustls::crypto::aws_lc_rs::default_provider()`,
/// but with the addition of X25519Kyber768Draft00 key exchange.
///
/// Returns `NULL` if the provider initialization panics.
#[no_mangle]
pub extern "C" fn rustls_post_quantum_ffi_crypto_provider() -> *const ffi::c_void {
    panic::catch_unwind(|| {
        let provider = rustls_post_quantum::provider();
        Arc::into_raw(Arc::new(provider)) as *const ffi::c_void
    })
    .unwrap_or(ptr::null())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_construct() {
        let provider = super::rustls_post_quantum_ffi_crypto_provider();
        assert!(!provider.is_null());
    }
}
