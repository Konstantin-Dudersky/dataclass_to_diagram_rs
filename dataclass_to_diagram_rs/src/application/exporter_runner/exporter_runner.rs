use super::traits::{IFileWriter, IFolderManipulation};

use super::super::exporters;
use crate::domain::models::Diagrams;

pub struct ExporterRunner {}

struct ExportedDiagram {
    export: String,
    filename: String,
}

impl ExporterRunner {
    pub fn new(
        file_writer: Box<dyn IFileWriter>,
        folder_manipulation: Box<dyn IFolderManipulation>,
        dias: Vec<Diagrams>,
    ) {
        folder_manipulation.remove_dir_all();
        folder_manipulation.create_dir();

        let exported_diagram = dias.iter().map(|dia| {
            let (name, content) = export(dia);
            ExportedDiagram {
                filename: format!("{}.puml", name),
                export: content,
            }
        });
        // записываем текстовые файлы
        let _ = exported_diagram
            .map(|ed| file_writer.write(&ed.filename, &ed.export).unwrap())
            .collect::<()>();
    }
}

fn export(dia: &Diagrams) -> (String, String) {
    match dia {
        Diagrams::C4(dia) => {
            (dia.name.clone(), exporters::c4_to_puml::export(dia))
        }
        Diagrams::StateMachine => todo!(),
    }
}
