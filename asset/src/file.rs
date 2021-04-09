use uuid::Uuid;

#[derive(Debug)]
pub struct AssetFile<'a, const N: usize> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8; N],
}
