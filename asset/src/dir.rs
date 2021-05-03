use super::file::AssetFile;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssetDir<'a> {
    pub id: Uuid,
    pub name: String,
    pub dirs: Vec<AssetDir<'a>>,
    pub files: Vec<AssetFile<'a>>,
}

impl<'a> AssetDir<'a> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn dirs(&self) -> &Vec<AssetDir> {
        &self.dirs
    }

    pub fn files(&self) -> &Vec<AssetFile> {
        &self.files
    }
}
