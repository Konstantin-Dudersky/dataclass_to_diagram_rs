pub trait IExportDiagram {
    fn export(&self) -> String;
    fn get_filename(&self) -> String;
}
