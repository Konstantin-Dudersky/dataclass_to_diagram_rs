use crate::application::text_form_saver::exporter_runner::ExporterRunner;

use crate::domain::models::Diagrams;
use crate::infrastructure::{
    file_writer::FileWriter, folder_manipulation::FolderManipulation,
};

use super::exporters::ExportedDiagram;

const FOLDER: &str = "diagrams/";

pub struct Runner {}

impl Runner {
    pub fn new(dias: Vec<Diagrams>) {
        // зависимости
        let file_writer = Box::new(FileWriter::new(FOLDER));

        // экспортируем диаграммы
        let dias: Vec<ExportedDiagram> =
            dias.iter().map(|dia| ExportedDiagram::new(dia)).collect();

        ExporterRunner::new(
            file_writer,
            Box::new(FolderManipulation::new(FOLDER)),
            &dias,
        );
    }
}
