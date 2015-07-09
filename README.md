# 32-bit CI for Rust on Travis

This repo is an example of how to setup a Rust project to run a matrix
CI build on travis.

[![Build Status](https://travis-ci.org/carllerche/travis-rust-matrix.svg?branch=master)](https://travis-ci.org/carllerche/travis-rust-matrix)

## Rationale

It's a good idea to run tests on all supported platforms. While Rust
supports 32 bit platforms, Travis currently does not. However, it is
possible to cross compile 32 bit builds. This project shows how to do
it.

## Usage

Add the following to your `.travis.yml` file:

```yaml
env:
  - ARCH=x86_64
  - ARCH=i686

script:
  - curl -sSL https://raw.githubusercontent.com/carllerche/travis-rust-matrix/master/test | bash
```

[Example](.travis.yml)

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
