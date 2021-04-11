use uuid::Uuid;

#[derive(Debug)]
pub struct AssetFile<'a, const N: usize> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8; N],
}

pub trait IAssetFile {
    fn name(&self) -> String;
    fn content(&self) -> Vec<u8>;
}

impl<'a, const N: usize> IAssetFile for AssetFile<'a, N> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn content(&self) -> Vec<u8> {
        self.data_ref.to_vec()
    }
}
