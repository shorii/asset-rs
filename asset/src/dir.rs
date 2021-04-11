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
    fn dirs(&self) -> &Vec<Box<dyn IAssetDir>>;
    fn files(&self) -> &Vec<Box<dyn IAssetFile>>;
}

impl IAssetDir for AssetDir {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn dirs(&self) -> &Vec<Box<dyn IAssetDir>> {
        &self.dirs
    }

    fn files(&self) -> &Vec<Box<dyn IAssetFile>> {
        &self.files
    }
}
