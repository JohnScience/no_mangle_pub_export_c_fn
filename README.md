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

# Seralization & Deserialization

All structures in this library implement [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits from [https://docs.rs/serde/latest/serde/#]. Because of that you can convert into many data formats supporting serde.

## Data formats

- [JSON], the ubiquitous JavaScript Object Notation used by many HTTP APIs.
- [Bincode], a compact binary format
  used for IPC within the Servo rendering engine.
- [CBOR], a Concise Binary Object Representation designed for small message
  size without the need for version negotiation.
- [YAML], a self-proclaimed human-friendly configuration language that ain't
  markup language.
- [MessagePack], an efficient binary format that resembles a compact JSON.
- [TOML], a minimal configuration format used by [Cargo].
- [Pickle], a format common in the Python world.
- [RON], a Rusty Object Notation.
- [BSON], the data storage and network transfer format used by MongoDB.
- [Avro], a binary format used within Apache Hadoop, with support for schema
  definition.
- [JSON5], a superset of JSON including some productions from ES5.
- [Postcard], a no\_std and embedded-systems friendly compact binary format.
- [URL] query strings, in the x-www-form-urlencoded format.
- [Envy], a way to deserialize environment variables into Rust structs.
  *(deserialization only)*
- [Envy Store], a way to deserialize [AWS Parameter Store] parameters into
  Rust structs. *(deserialization only)*
- [S-expressions], the textual representation of code and data used by the
  Lisp language family.
- [D-Bus]'s binary wire format.
- [FlexBuffers], the schemaless cousin of Google's FlatBuffers zero-copy serialization format.
- [DynamoDB Items], the format used by [rusoto_dynamodb] to transfer data to
  and from DynamoDB.

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

[JSON]: https://github.com/serde-rs/json
[Bincode]: https://github.com/servo/bincode
[CBOR]: https://github.com/enarx/ciborium
[YAML]: https://github.com/dtolnay/serde-yaml
[MessagePack]: https://github.com/3Hren/msgpack-rust
[TOML]: https://github.com/alexcrichton/toml-rs
[Pickle]: https://github.com/birkenfeld/serde-pickle
[RON]: https://github.com/ron-rs/ron
[BSON]: https://github.com/zonyitoo/bson-rs
[Avro]: https://github.com/flavray/avro-rs
[JSON5]: https://github.com/callum-oakley/json5-rs
[Postcard]: https://github.com/jamesmunns/postcard
[URL]: https://docs.rs/serde_qs
[Envy]: https://github.com/softprops/envy
[Envy Store]: https://github.com/softprops/envy-store
[Cargo]: https://doc.rust-lang.org/cargo/reference/manifest.html
[AWS Parameter Store]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-paramstore.html
[S-expressions]: https://github.com/rotty/lexpr-rs
[D-Bus]: https://docs.rs/zvariant
[FlexBuffers]: https://github.com/google/flatbuffers/tree/master/rust/flexbuffers
[DynamoDB Items]: https://docs.rs/serde_dynamo
[rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
