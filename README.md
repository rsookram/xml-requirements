# xml-requirements

## Installation

Currently, pre-compiled binaries of xml-requirements aren't being distributed.
You can install it with
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by
running

```
cargo install --git https://github.com/rsookram/xml-requirements
```

## Usage

```
USAGE:
    xml-requirements --config <config> [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Path to toml configuration file

ARGS:
    <FILE>...    Path of XML files to check
```

## Building

xml-requirements can be built from source by cloning this repository and using
Cargo.

```
$ git clone https://github.com/rsookram/xml-requirements
$ cd xml-requirements
$ cargo build --release
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
