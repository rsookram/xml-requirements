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

The first thing you'll need is a configuration file which tells
xml-requirements what to look for. This is a
[TOML](https://github.com/toml-lang/toml) file which specifies a top-level
table where the keys are the XML tags to check, and the values are a key-value
pair with:

  - `required` as the key and
  - an array of strings as the value

The strings are the attributes which must be present.

Here's what a configuration file may look like:

```toml
# All <LinearLayout> tags must have an "android:orientation" attribute
[LinearLayout]
required = [
  # If "android:orientation" isn't present in any of the specified files, then
  # a warning will be printed
  "android:orientation",
]

[TextView]
required = [
  "style", # For attributes without a namespace, only include the name

  # Every value in a `required` array must be present in the checked files. So
  # every TextView must contain both "style" AND "android:textAppearance".
  "android:textAppearance",
]
```

With that, here's an example of how to run xml-requirements from the command
line:

```shell
$ xml-requirements --config path/to/config.toml path/to/file0.xml path/to/file1.xml
```

And the full `--help` output for more info:

```
USAGE:
    xml-requirements --config <config> [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Path to TOML configuration file

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
