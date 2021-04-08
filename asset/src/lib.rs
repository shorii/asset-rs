use std::convert::AsRef;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct Asset<const N: usize> {
    id: Uuid,
    name: String,
    data: [u8; N],
}

impl<const N: usize> Asset<N> {
    pub fn new(name: &str, data: [u8; N]) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid,
            name: name.to_string(),
            data: data,
        }
    }
}
