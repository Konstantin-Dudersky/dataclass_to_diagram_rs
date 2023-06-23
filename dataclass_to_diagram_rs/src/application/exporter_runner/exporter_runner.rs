use std::fs;

use super::super::exporters::traits::IExporter;

pub struct ExporterRunner {}

struct ExportedDiagram {
    export: String,
    filename: String,
}

impl ExporterRunner {
    pub fn new(dias: Vec<Box<dyn IExporter>>) {
        fs::remove_dir_all("diagrams").unwrap_or_default();
        fs::create_dir("diagrams").unwrap();

        let exported_diagram = dias.iter().map(|dia| ExportedDiagram {
            filename: dia.get_filename(),
            export: dia.export(),
        });
        // записываем текстовые файлы
        let _ = exported_diagram
            .map(|ed| {
                fs::write(format!("diagrams/{}", ed.filename), ed.export)
                    .unwrap()
            })
            .collect::<()>();
    }
}
