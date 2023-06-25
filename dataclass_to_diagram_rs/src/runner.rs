use crate::application::{
    exporter_runner::exporter_runner::ExporterRunner,
    exporters::traits::IExportDiagram,
};

use crate::infrastructure::{
    file_writer::FileWriter, folder_manipulation::FolderManipulation,
};

const FOLDER: &str = "diagrams/";

pub struct Runner {}

impl Runner {
    pub fn new(dias: Vec<Box<dyn IExportDiagram>>) {
        ExporterRunner::new(
            Box::new(FileWriter::new(FOLDER)),
            Box::new(FolderManipulation::new(FOLDER)),
            dias,
        );
    }
}
