# flatpak-rs
Flatpak library for Rust.

![Tests status](https://github.com/louib/flatpak-rs/workflows/tests/badge.svg)
![Code formatting](https://github.com/louib/flatpak-rs/workflows/formatting/badge.svg)
![Documentation](https://github.com/louib/flatpak-rs/workflows/doc/badge.svg)
[![dependency status](https://deps.rs/repo/github/louib/flatpak-rs/status.svg)](https://deps.rs/repo/github/louib/flatpak-rs)
[![Crates.io version](https://img.shields.io/crates/v/flatpak-rs?style=flat-square)](https://crates.io/crates/flatpak-rs)
[![License file](https://img.shields.io/github/license/louib/flatpak-rs)](https://github.com/louib/flatpak-rs/blob/master/LICENSE)

This library offers functions to parse and dump [flatpak](https://github.com/flatpak/flatpak) application,
module or source manifests. The goal of the library is to be compliant with what
[`flatpak-builder`](https://github.com/flatpak/flatpak-builder) supports.

See the [API documentation](https://docs.rs/flatpak/) for this library.

## Installation
Add the library to your `Cargo.toml`:
```ignore
flatpak = "0"
```

## Usage
All three denominations of Flatpak manifests can be parsed using this library,
using the `FlatpakApplication`, `FlatpakModule` and `FlatpakSource` structs.

### Parse from a string
```
use flatpak_rs::application::FlatpakApplication;
use flatpak_rs::format::FlatpakManifestFormat;

let manifest = r###"
    app-id: net.louib.flatpak-rs
    runtime: org.gnome.Platform
    runtime-version: "3.36"
    sdk: org.gnome.Sdk
    command: flatpak-rs
    tags: ["nightly"]
    modules:
      -
        name: "flatpak-rs"
        buildsystem: simple
        cleanup: [ "*" ]
        config-opts: []
        sources:
          -
            type: git
            url: https://github.com/louib/flatpak-rs.git
            branch: master
      -
        "shared-modules/linux-audio/lv2.json"
"###;

let application = FlatpakApplication::parse(FlatpakManifestFormat::YAML, manifest).unwrap();

assert_eq!(&application.app_id, "net.louib.flatpak-rs");
assert_eq!(application.modules.len(), 2 as usize);

println!("Parsed application manifest for {}.", &application.app_id);
```
### Parse from a file
```
use std::env;

use flatpak_rs::application::FlatpakApplication;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a flatpak application manifest to parse.");
        return;
    }
    let manifest_path = &args[1];

    let application = FlatpakApplication::load_from_file(manifest_path.clone()).unwrap();
    println!("Parsed application manifest for {}.", &application.get_id());
}

```

Note that this library is aliased as both `flatpak` and `flatpak-rs` on crates.io.

## License
MIT
