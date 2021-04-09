use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use syn::Ident;
use uuid::Uuid;

pub struct AssetBuilder {}

impl AssetBuilder {
    fn build_asset_file<P: AsRef<Path>>(src: P, name: &str) -> TokenStream {
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
        // TODO use this ident
        let init_asset_fn_ident = format_ident!("init_asset{}", data_ident);
        quote! {
            const #data_ident: [u8; #size] = hex!(#bytes);
            pub fn init_asset() -> AssetFile<'static, #size>{
                let parsed_uuid = Uuid::parse_str(#uuid).expect("invalid uuid");
                AssetFile {
                    id: parsed_uuid,
                    name: #name.to_string(),
                    data_ref: &#data_ident,
                }
            }
        }
    }

    pub fn build<P: AsRef<Path>>(src: P) -> TokenStream {
        let use_statement = quote! {
            use hex_literal::hex;
            use asset::file::AssetFile;
            use uuid::Uuid;
        };

        let path: &Path = src.as_ref();
        if let Some(os_name) = path.file_name() {
            if let Some(name) = os_name.to_str() {
                if path.is_file() {
                    let init_code = Self::build_asset_file(path, &name);
                    return quote! {
                        #use_statement
                        #init_code
                    };
                } else {
                    panic!("Cannot convert directory to Asset yet.");
                }
            }
        }
        panic!("Failed to retrieve filename");
    }
}
