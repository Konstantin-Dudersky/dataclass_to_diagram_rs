use super::traits::{IFileWriter, IFolderManipulation};

use super::super::exporters::ExportedDiagram;

pub struct ExporterRunner {}

impl ExporterRunner {
    pub fn new(
        file_writer: Box<dyn IFileWriter>,
        folder_manipulation: Box<dyn IFolderManipulation>,
        dias: &Vec<ExportedDiagram>,
    ) {
        folder_manipulation.remove_dir_all();
        folder_manipulation.create_dir();

        let _ = dias
            .iter()
            .map(|ed| file_writer.write(&ed.name, &ed.content).unwrap())
            .collect::<()>();
    }
}
