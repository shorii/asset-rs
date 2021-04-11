use super::file::IAssetFile;
use uuid::Uuid;

pub struct AssetDir {
    pub id: Uuid,
    pub name: String,
    pub dirs: Vec<Box<dyn IAssetDir>>,
    pub files: Vec<Box<dyn IAssetFile>>,
}

pub trait IAssetDir {
    fn name(&self) -> String;
}

impl IAssetDir for AssetDir {
    fn name(&self) -> String {
        self.name.clone()
    }
}
