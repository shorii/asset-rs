use uuid::Uuid;

#[derive(Debug)]
pub struct AssetFile<'a, const N: usize> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8; N],
}

pub trait IAssetFile {
    fn name(&self) -> String;
}

impl<'a, const N: usize> IAssetFile for AssetFile<'a, N> {
    fn name(&self) -> String {
        self.name.clone()
    }
}
