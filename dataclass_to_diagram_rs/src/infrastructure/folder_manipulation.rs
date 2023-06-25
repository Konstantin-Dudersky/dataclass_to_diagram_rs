use std::fs;

use crate::application::exporter_runner::traits::IFolderManipulation;

pub struct FolderManipulation {
    folder: String,
}

impl FolderManipulation {
    pub fn new(folder: &str) -> Self {
        Self {
            folder: String::from(folder),
        }
    }
}

impl IFolderManipulation for FolderManipulation {
    fn remove_dir_all(&self) {
        fs::remove_dir_all(&self.folder).unwrap_or_default();
    }

    fn create_dir(&self) {
        fs::create_dir(&self.folder).unwrap();
    }
}
