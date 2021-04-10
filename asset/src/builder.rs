use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use syn::Ident;
use uuid::Uuid;

pub struct AssetBuilder {}

struct FnTokenStreamSet {
    define: TokenStream,
    call: TokenStream,
}

impl AssetBuilder {
    fn build_asset_file<P: AsRef<Path>>(src: P, name: &str) -> FnTokenStreamSet {
        let file = File::open(src).unwrap();
        let mut reader = BufReader::new(file);
        let mut data = vec![];
        reader.read_to_end(&mut data).unwrap();
        let bytes = data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let size = Literal::usize_unsuffixed(data.len());
        let uuid = Uuid::new_v4().to_simple().to_string();
        let data_ident = format_ident!(
            "__{}",
            Ident::new(
                &format!("DATA_{}", uuid.to_ascii_uppercase()),
                Span::call_site()
            )
        );
        let init_asset_fn_ident = format_ident!("init_asset{}", data_ident);
        FnTokenStreamSet {
            define: quote! {
                const #data_ident: [u8; #size] = hex!(#bytes);
                pub fn #init_asset_fn_ident() -> Box<dyn Asset> {
                    let parsed_uuid = Uuid::parse_str(#uuid).expect("invalid uuid");
                    let file = AssetFile {
                        id: parsed_uuid,
                        name: #name.to_string(),
                        data_ref: &#data_ident,
                    };
                    Box::new(file)
                }
            },
            call: quote! {
                #init_asset_fn_ident()
            },
        }
    }

    pub fn build<P: AsRef<Path>>(src: P) -> TokenStream {
        let use_statement = quote! {
            use hex_literal::hex;
            use asset::file::AssetFile;
            use asset::types::Asset;
            use uuid::Uuid;
        };
        let mut defines = vec![];
        let mut calls = vec![];
        let path: &Path = src.as_ref();
        if let Some(os_name) = path.file_name() {
            if let Some(name) = os_name.to_str() {
                if path.is_file() {
                    let fn_token_stream_set = Self::build_asset_file(path, &name);
                    defines.push(fn_token_stream_set.define);
                    calls.push(fn_token_stream_set.call);
                } else {
                    panic!("Cannot convert directory to Asset yet.");
                }
            }
        }
        quote! {
            #use_statement
            #(#defines)*
            fn init_asset() -> Box<dyn Asset>{
                #(#calls)*
            }
        }
    }
}
