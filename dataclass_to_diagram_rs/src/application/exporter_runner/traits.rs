pub trait IFileWriter {
    #[allow(unused_variables)]
    fn write(
        &self,
        filename: &str,
        file_content: &str,
    ) -> Result<(), std::io::Error>;
}

pub trait IFolderManipulation {
    fn remove_dir_all(&self);
    fn create_dir(&self);
}
