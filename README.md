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