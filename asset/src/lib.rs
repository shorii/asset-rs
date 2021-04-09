use uuid::Uuid;

#[derive(Debug)]
pub struct Asset<'a, const N: usize> {
    pub id: Uuid,
    pub name: String,
    pub data_ref: &'a [u8; N],
}

impl<'a, const N: usize> Asset<'a, N> {
    pub fn new(name: &str, data_ref: &'a [u8; N]) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid,
            name: name.to_string(),
            data_ref: data_ref,
        }
    }
}
