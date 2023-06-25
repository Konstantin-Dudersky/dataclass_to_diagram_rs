use super::{
    super::exporters::traits::IExportDiagram,
    traits::{IFileWriter, IFolderManipulation},
};

pub struct ExporterRunner {}

struct ExportedDiagram {
    export: String,
    filename: String,
}

impl ExporterRunner {
    pub fn new(
        file_writer: Box<dyn IFileWriter>,
        folder_manipulation: Box<dyn IFolderManipulation>,
        dias: Vec<Box<dyn IExportDiagram>>,
    ) {
        folder_manipulation.remove_dir_all();
        folder_manipulation.create_dir();

        let exported_diagram = dias.iter().map(|dia| ExportedDiagram {
            filename: dia.get_filename(),
            export: dia.export(),
        });
        // записываем текстовые файлы
        let _ = exported_diagram
            .map(|ed| file_writer.write(&ed.filename, &ed.export).unwrap())
            .collect::<()>();
    }
}
