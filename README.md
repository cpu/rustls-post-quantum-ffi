<p align="center">
  <img width="460" height="300" src="https://raw.githubusercontent.com/rustls/rustls/main/admin/rustls-logo-web.png">
</p>

<p align="center">
Rustls is a modern TLS library written in Rust.
</p>

> [!WARNING]
> This is an experimental **demo** and not production ready.

# rustls-post-quantum-ffi

This crate provides experimental [rustls-ffi] C bindings for a crypto provider
that enables  [X25519Kyber768Draft00] post-quantum key exchange using [rustls-post-quantum].

It primarily serves as an example of how a `CryptoProvider` other than the two
options built in to `rustls` and `rustls-ffi` (`aws-lc-rs` or `*ring*`) can be
used with `rustls-ffi`.

> [!NOTE]
> Crypto provider support has not yet landed in `rustls-ffi`. This demo depends
> on unmerged work from [rustls/rustls-ffi#441]

[rustls-ffi]: https://github.com/rustls/rustls-ffi
[rustls-post-quantum]:  https://crates.io/crates/rustls-post-quantum
[X25519Kyber768Draft00]: https://datatracker.ietf.org/doc/draft-tls-westerbaan-xyber768d00/03/
[rustls/rustls-ffi#441]: https://github.com/rustls/rustls-ffi/pull/441
