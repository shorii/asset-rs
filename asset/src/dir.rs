use super::file::AssetFile;
use anyhow::{anyhow, bail, Result};
use std::path::Path;
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

    pub fn find<P: AsRef<Path>>(&self, path: P) -> Result<&AssetFile> {
        let path = path.as_ref();
        let elements = path.iter().map(|p| p.to_str()).collect::<Vec<_>>();
        let mut filtered = elements
            .iter()
            .filter(|p| **p != None)
            .map(|p| String::from(p.unwrap()))
            .collect::<Vec<String>>();
        if elements.len() != filtered.len() {
            bail!("file not found ");
        }
        let mut current_directory = self;
        if filtered.len() <= 1 {
            bail!("file not found");
        }
        if filtered.len() == 2 {
            let filename = filtered[1].clone();
            for file in current_directory.files() {
                if file.name() == filename {
                    return Ok(file);
                }
            }
        } else {
            let filename = filtered.pop().unwrap();
            for dirname in filtered {
                for dir in current_directory.dirs() {
                    if dir.name() == dirname {
                        current_directory = dir;
                    }
                }
            }
            for file in current_directory.files() {
                if file.name() == filename {
                    return Ok(file);
                }
            }
        }
        Err(anyhow!("Failed to find file"))
    }
}
