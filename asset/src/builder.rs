use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::default::Default;
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

impl Default for FnTokenStreamSet {
    fn default() -> Self {
        Self {
            define: quote! {},
            call: quote! {},
        }
    }
}

impl AssetBuilder {
    fn build_asset_dir<P: AsRef<Path>>(src: P, name: &str) -> FnTokenStreamSet {
        let path = src.as_ref();
        let entries = path.read_dir().unwrap();
        let mut files: Vec<FnTokenStreamSet> = vec![];
        let mut dirs: Vec<FnTokenStreamSet> = vec![];
        for e in entries {
            let entry = e.unwrap();
            let os_entry_name = entry.file_name();
            if let Some(entry_name) = os_entry_name.to_str() {
                let entry_path = path.join(entry_name);
                let file_type = entry.file_type().unwrap();
                if file_type.is_file() {
                    files.push(AssetBuilder::build_asset_file(entry_path, entry_name));
                } else if file_type.is_dir() {
                    dirs.push(AssetBuilder::build_asset_dir(entry_path, entry_name));
                } else {
                    panic!("Unsupported file type");
                }
            }
        }

        let uuid = Uuid::new_v4().to_simple().to_string();
        let data_ident = format_ident!(
            "__{}",
            Ident::new(
                &format!("DIR_{}", uuid.to_ascii_uppercase()),
                Span::call_site()
            )
        );
        let init_asset_fn_ident = format_ident!("init_asset{}", data_ident);

        // define
        let file_defines = files.iter().map(|f| f.define.clone()).collect::<Vec<_>>();
        let file_calls = files.iter().map(|f| f.call.clone()).collect::<Vec<_>>();
        let dir_defines = dirs.iter().map(|f| f.define.clone()).collect::<Vec<_>>();
        let dir_calls = dirs.iter().map(|d| d.call.clone()).collect::<Vec<_>>();
        let define = quote! {
            #(#file_defines)*
            #(#dir_defines)*

            fn #init_asset_fn_ident() -> AssetDir<'static> {
                let parsed_uuid = Uuid::parse_str(#uuid).expect("invalid uuid");
                AssetDir {
                    id: parsed_uuid,
                    name: #name.to_string(),
                    dirs: vec![
                        #(#dir_calls),*
                    ],
                    files: vec![
                        #(#file_calls),*
                    ],
                }
            }
        };

        // call
        let call = quote! {
            #init_asset_fn_ident()
        };

        FnTokenStreamSet {
            define: define,
            call: call,
        }
    }

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
                pub fn #init_asset_fn_ident() -> AssetFile<'static> {
                    let parsed_uuid = Uuid::parse_str(#uuid).expect("invalid uuid");
                    AssetFile {
                        id: parsed_uuid,
                        name: #name.to_string(),
                        data_ref: &#data_ident,
                    }
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
            use asset::dir::AssetDir;
            use uuid::Uuid;
        };
        let path: &Path = src.as_ref();
        if let Some(os_name) = path.file_name() {
            if let Some(name) = os_name.to_str() {
                if path.is_file() {
                    // output:
                    //
                    // use hex_literal::hex;
                    // use asset::file::{AssetFile, IAssetFile};
                    // use asset::dir::{AssetDir, IAssetDir};
                    // use uuid::Uuid;
                    //
                    // const __DATA_0535ACC556964E608B118E3F5DB094B9: [u8; 13] = hex!("666f6f2062617220717175780a");
                    //
                    // pub fn init_asset__DATA_0535ACC556964E608B118E3F5DB094B9() -> Box<dyn IAssetFile> {
                    //     let parsed_uuid = Uuid::parse_str("0535acc556964e608b118e3f5db094b9").expect("invalid uuid");
                    //     let file = AssetFile {
                    //         id: parsed_uuid,
                    //         name: "test.txt".to_string(),
                    //         data_ref: &__DATA_0535ACC556964E608B118E3F5DB094B9
                    //     };
                    //     Box::new(file)
                    // }
                    //
                    // fn init_asset() -> Box <dyn IAssetFile> {
                    //     init_asset__DATA_0535ACC556964E608B118E3F5DB094B9()
                    // }
                    let fn_token_stream_set = Self::build_asset_file(path, &name);
                    let define = fn_token_stream_set.define;
                    let call = fn_token_stream_set.call;
                    return quote! {
                        #use_statement
                        #define
                        pub fn init_asset() -> AssetFile<'static> {
                            #call
                        }
                    };
                } else {
                    // output:
                    //
                    // use asset::dir::{AssetDir, IAssetDir};
                    // use asset::file::{AssetFile, IAssetFile};
                    // use hex_literal::hex;
                    // use uuid::Uuid;
                    //
                    // const __DATA_3408EA068EF6496DBFF80C3FC586DD4F: [u8; 13] = hex!("666f6f2062617220717175780a");
                    //
                    // pub fn init_asset__DATA_3408EA068EF6496DBFF80C3FC586DD4F() -> Box<dyn IAssetFile> {
                    //     let parsed_uuid = Uuid::parse_str("3408ea068ef6496dbff80c3fc586dd4f").expect("invalid uuid");
                    //     let file = AssetFile {
                    //         id: parsed_uuid,
                    //         name: "test.txt".to_string(),
                    //         data_ref: &__DATA_3408EA068EF6496DBFF80C3FC586DD4F,
                    //     };
                    //     Box::new(file)
                    // }
                    //
                    // fn init_asset__DIR_31C82C5F81194FF5B423FF502A53199F() -> Box<dyn IAssetDir> {
                    //     let parsed_uuid = Uuid::parse_str("31c82c5f81194ff5b423ff502a53199f").expect("invalid uuid");
                    //     let dir = AssetDir {
                    //         id: parsed_uuid,
                    //         name: "assets".to_string(),
                    //         dirs: vec![],
                    //         files: vec![init_asset__DATA_3408EA068EF6496DBFF80C3FC586DD4F()],
                    //     };
                    //     Box::new(dir)
                    // }
                    //
                    // pub fn init_asset() -> Box<dyn IAssetDir> {
                    //     init_asset__DIR_31C82C5F81194FF5B423FF502A53199F()
                    // }
                    let fn_token_stream_set = Self::build_asset_dir(path, &name);
                    let define = fn_token_stream_set.define;
                    let call = fn_token_stream_set.call;
                    return quote! {
                        #use_statement
                        #define
                        pub fn init_asset() -> AssetDir<'static> {
                            #call
                        }
                    };
                }
            }
        }
        panic!("Failed to generate initcode.");
    }
}
