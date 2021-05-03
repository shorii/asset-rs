use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssetFile<'a> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8],
}

impl<'a> AssetFile<'a> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn content(&self) -> Vec<u8> {
        self.data_ref.to_vec()
    }
}
