Example Rust project

This repo is an example of how to setup a Rust project to run a matrix
CI build on travis.

## Rationale

It's a good idea to run tests on all supported platforms. While Rust
supports 32 bit platforms, Travis currently does not. However, it is
possible to cross compile 32 bit builds. This project shows how to do
it.
