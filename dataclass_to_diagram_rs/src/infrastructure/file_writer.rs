use std::fs;

use crate::application::exporter_runner::traits::IFileWriter;

pub struct FileWriter {
    folder: String,
}

impl FileWriter {
    pub fn new(folder: &str) -> Self {
        Self {
            folder: String::from(folder),
        }
    }
}

impl IFileWriter for FileWriter {
    fn write(
        &self,
        filename: &str,
        file_content: &str,
    ) -> Result<(), std::io::Error> {
        fs::write(
            format!(
                "{folder}{filename}",
                folder = self.folder,
                filename = filename
            ),
            file_content,
        )
    }
}
