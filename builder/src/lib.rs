use proc_macro2::{Literal, Span};
use quote::{format_ident, quote};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use syn::Ident;
use uuid::Uuid;

pub struct Builder {}

impl Builder {
    pub fn build<P: AsRef<Path>>(src: P) -> String {
        let use_statement = quote! {
            use hex_literal::hex;
            use asset::Asset;
            use uuid::Uuid;
        };

        let path: &Path = src.as_ref();
        if let Some(os_name) = path.file_name() {
            if let Some(name) = os_name.to_str() {
                if path.is_file() {
                    let file = File::open(path).unwrap();
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
                        Ident::new(&format!("data_{}", uuid), Span::call_site())
                    );
                    // TODO use this ident
                    let init_asset_fn_ident = format_ident!("init_asset{}", data_ident);
                    let init_code = quote! {
                        #use_statement
                        const #data_ident: [u8; #size] = hex!(#bytes);
                        pub fn init_asset() -> Asset<'static, #size>{
                            let parsed_uuid = Uuid::parse_str(#uuid).expect("invalid uuid");
                            Asset {
                                id: parsed_uuid,
                                name: #name.to_string(),
                                data_ref: &#data_ident,
                            }
                        }
                    };
                    return init_code.to_string();
                } else {
                    panic!("Cannot convert directory to Asset yet.");
                }
            }
        }
        panic!("Failed to retrieve filename");
    }
}
