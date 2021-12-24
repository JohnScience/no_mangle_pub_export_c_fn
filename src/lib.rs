// What are no mangle pub export C functions?
// https://docs.rust-embedded.org/book/interoperability/rust-with-c.html#no_mangle
// The library can be generalized for arbitrary calling conventions
// https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions

use std::io::Read;

use proc_macro2::{LineColumn, Span};
use syn::{spanned::Spanned, visit::Visit, Visibility};
use walkdir::WalkDir;

// https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

#[derive(Debug)]
struct NoManglePubExportCFnEnds {
    start: LineColumn,
    end: LineColumn,
}

impl NoManglePubExportCFnEnds {
    fn new(span: &Span) -> Self {
        Self {
            start: span.start(),
            end: span.end(),
        }
    }
}

#[derive(Default, Debug)]
struct NoManglePubExportCFns {
    no_mangle_pub_export_c_fn_vec: Vec<NoManglePubExportCFnEnds>,
}

// https://docs.rs/syn/latest/syn/visit/index.html
impl<'ast> Visit<'ast> for NoManglePubExportCFns {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        match &node.vis {
            Visibility::Public(_) => (),
            _ => return,
        };

        // https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions
        if let Some(calling_convention)  = node.sig.abi.as_ref()
        .and_then(|abi| abi.name.as_ref())
        .map(|str_lit| str_lit.value()) {
            if calling_convention != "C" { return }
        }
        

        if !node
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("no_mangle"))
        {
            return;
        };

        self.no_mangle_pub_export_c_fn_vec
            .push(NoManglePubExportCFnEnds::new(&{
                let span: proc_macro2::Span = node.span();
                span
            }));
    }
}

#[derive(Debug)]
pub struct ParsedFile {
    path: String,
    no_mangle_pub_export_c_fns: NoManglePubExportCFns,
}

pub fn parse_for_no_mangle_pub_extern_c_fns(crate_root: &str) -> Vec<ParsedFile> {
    // With prior information, the buffer could be preallocated
    let mut buffer = String::new();

    // All errors are skipped. In the hindsight, the solution with ? would be better
    WalkDir::new(format!("{}{}src", crate_root, std::path::MAIN_SEPARATOR))
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".rs"))
        // BufReader is unnecessary bc the files are read only once
        // https://doc.rust-lang.org/std/io/struct.BufReader.html
        .filter_map(|rust_file| {
            let path = rust_file.path();
            std::fs::File::open(path)
                .map(|file: std::fs::File| (path.to_string_lossy().into_owned(), file))
                .ok()
        })
        .filter_map(|(path, mut file): (String, std::fs::File)| {
            file.read_to_string(&mut buffer)
                .map(|_byted_read| {
                    let file = syn::parse_file(&buffer);
                    buffer.truncate(0);
                    (path, file)
                })
                .ok()
        })
        .filter_map(|(path, res_file): (String, syn::Result<syn::File>)| {
            res_file.ok().map(|file| (path, file))
        })
        .fold(
            Vec::<ParsedFile>::new(),
            |mut parsed_files: Vec<ParsedFile>, (path, file): (String, syn::File)| {
                parsed_files.push(ParsedFile {
                    path,
                    no_mangle_pub_export_c_fns: {
                        let mut no_mangle_pub_export_c_fns = NoManglePubExportCFns::default();
                        no_mangle_pub_export_c_fns.visit_file(&file);
                        no_mangle_pub_export_c_fns
                    },
                });
                parsed_files
            },
        )
}
