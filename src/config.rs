use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Album {
    #[serde(skip)]
    pub path: PathBuf,

    pub title: String,
    pub desc: String,

    #[serde(default)]
    pub javascript: bool,
    pub photos: Vec<Photo>,
}

impl Album {
    pub fn load(album_dir: impl AsRef<Path>) -> Result<Self> {
        let album_config = album_dir.as_ref().join("album.yml");

        Ok(Self {
            path: album_dir.as_ref().into(),
            ..serde_yaml::from_str(&fs::read_to_string(&album_config)?)?
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Photo {
    pub name: String,
    pub desc: Option<String>,
}

impl Photo {
    pub fn slugify(&self) -> Result<String> {
        Ok(slug::slugify(
            Path::new(&self.name)
                .file_stem()
                .and_then(|p| p.to_str())
                .ok_or_else(|| anyhow!("invalid-looking photo name: {:?}", &self.name))?,
        ))
    }
}
