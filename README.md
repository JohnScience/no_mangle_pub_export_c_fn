# A library for extracting #\[no_mangle\] pub extern "C" functions

In order to expose a function with C binary interface for interoperability with other programming languages,
Rust developers should use #\[no_mangle\] attribute and specify ABI for the declared function.

```rust
#[no_mangle]
pub extern "C" fn func_name () /* -> ... */ {
    // ...
}
```

You can read about it more [here](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html)
 
This library allows to perform [syn](https://crates.io/crates/syn)-driven parsing for obtaining the information about
location of these no-mangle-pub-extern-C functions.

# Example

`main.rs`

```rust
use no_mangle_pub_export_c_fn::{parse_for_no_mangle_pub_extern_c_fns, ParsedFile};

fn main() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let parsed_files: Vec<ParsedFile> = parse_for_no_mangle_pub_extern_c_fns(crate_root.as_str());
    println!("{:#?}", parsed_files);
}
```

`unused.rs`
```rust
#[no_mangle]
pub extern "C" fn s() {
    // test
}
```

# Output on Windows

```text
[
ParsedFile {
    path: "...\\no_mangle_pub_export_c_fn\\src\\lib.rs",
    no_mangle_pub_export_c_fns: NoManglePubExportCFns {
        no_mangle_pub_export_c_fn_vec: [],
    },
},
ParsedFile {
    path: "...\\no_mangle_pub_export_c_fn\\src\\main.rs",
    no_mangle_pub_export_c_fns: NoManglePubExportCFns {
        no_mangle_pub_export_c_fn_vec: [],
    },
},
ParsedFile {
    path: "...\\no_mangle_pub_export_c_fn\\src\\unused.rs",
    no_mangle_pub_export_c_fns: NoManglePubExportCFns {
        no_mangle_pub_export_c_fn_vec: [
            NoManglePubExportCFnEnds {
                start_line: 1,
                start_column: 0,
                end_line: 4,
                end_column: 1,
            },
        ],
    },
},
]
```

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>