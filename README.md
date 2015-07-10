# 32-bit CI for Rust on Travis & Appveyor

This repo is an example of how to setup a Rust project to run a matrix
CI build on Travis and Appveyor.

[![Build Status](https://travis-ci.org/carllerche/travis-rust-matrix.svg?branch=master)](https://travis-ci.org/carllerche/travis-rust-matrix)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/ask8rd2nquofihix?svg=true)](https://ci.appveyor.com/project/carllerche/travis-rust-matrix)

## Rationale

It's a good idea to run tests on all supported platforms. While Rust
supports 32 bit platforms, Travis currently does not. However, it is
possible to cross compile 32 bit builds. This project shows how to do
it.

If you want to support Windows, Appveyor provides a Windows CI system. I
also included how to setup a matrix build on Appveyor.

## Usage

Add the following to your `.travis.yml` file:

```yaml
env:
  - ARCH=x86_64
  - ARCH=i686

script:
  - curl -sSL https://raw.githubusercontent.com/carllerche/travis-rust-matrix/master/test | bash
```

## Examples

This project features a [simple .travis.yml](.travis.yml) file
configured to run 32-bit builds.

The [nix](https://github.com/carllerche/nix-rust) crate also uses this
strategy.

## Appveyor

Add the following to `appveyor.yml`

```yaml
install:
  - ps: Start-FileDownload "https://git.io/vq9LX"; . .\install.ps1

environment:
  matrix:
    - RUST_VERSION: 1.1.0
    - RUST_VERSION: beta
    - RUST_VERSION: nightly
```

See the included [appveyor.yml](appveyor.yml) for a fully working
example.

## License (MIT)

Copyright (c) 2015 Carl Lerche

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
