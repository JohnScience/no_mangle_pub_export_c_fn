use no_mangle_pub_export_c_fn::{parse_for_no_mangle_pub_extern_c_fns, ParsedFile};

mod lib;

fn main() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let parsed_files: Vec<ParsedFile> = parse_for_no_mangle_pub_extern_c_fns(crate_root.as_str());
    println!("{:#?}", parsed_files);
}
