use super::types::Asset;
use uuid::Uuid;

#[derive(Debug)]
pub struct AssetFile<'a, const N: usize> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8; N],
}

impl<'a, const N: usize> Asset for AssetFile<'a, N> {
    fn name(&self) -> String {
        self.name.clone()
    }
}
