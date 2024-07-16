#ifndef RUSTLS_POST_QUANTUM_H
#define RUSTLS_POST_QUANTUM_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Returns a pointer that may be cast to a rustls-ffi `*rustls_crypto_provider`.
 * You must use `rustls_crypto_provider_free` to free the pointer when done.
 *
 * The returned provider is equivalent to `rustls::crypto::aws_lc_rs::default_provider()`,
 * but with the addition of X25519Kyber768Draft00 key exchange.
 *
 * Returns `NULL` if the provider initialization panics.
 */
const void *rustls_post_quantum_ffi_crypto_provider(void);

#endif /* RUSTLS_POST_QUANTUM_H */
