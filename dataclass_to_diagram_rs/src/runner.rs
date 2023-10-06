use crate::application::exporter_runner::exporter_runner2::ExporterRunner;

use crate::domain::models::Diagrams;
use crate::infrastructure::{
    file_writer::FileWriter, folder_manipulation::FolderManipulation,
};

const FOLDER: &str = "diagrams/";

pub struct Runner {}

impl Runner {
    pub fn new(dias: Vec<Diagrams>) {
        ExporterRunner::new(
            Box::new(FileWriter::new(FOLDER)),
            Box::new(FolderManipulation::new(FOLDER)),
            dias,
        );
    }
}
