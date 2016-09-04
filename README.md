# qwk

| travis-ci | appveyor |
|-----------|----------|
| [![Build Status](https://travis-ci.org/booyaa/qwk.svg?branch=master)](https://travis-ci.org/booyaa/qwk) | [![Build Status](https://ci.appveyor.com/api/projects/status/github/booyaa/qwk)] |


A [Duck Duck Go](https://duckduckgo.com) client.

# Install

`cargo install qwk`

Binaries will be available in the [releases](https://github.com/booyaa/qwk/releases).

# Usage

`qwk foo`

Searches for 'foo' on Duck Duck Go and returns the Abstract Text or if `!bang` command the redirected url.

## Flags and options

```
USAGE:
    qwk [OPTIONS] <query>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <RECORDS>    Number of records to return


ARGS:
    <query>    Something to search
```

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
