# sync-resolve

This is a minimal fork of [`resolve`], which is no longer maintained. Most
users should probably prefer to use one of the asynchronous resolver crates,
some of which also provide synchronous API options, both for performance and
for being more maintained.

`sync-resolve` is a pure Rust implementation of the DNS protocol.

It also provides high level facilities for hostname resolution and address
reverse resolution.

[`resolve`]: https://github.com/murarth/resolve
