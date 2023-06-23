use crate::domain::models::state_machine::Diagram;

use super::super::traits::IExporter;

use super::diagram_to_puml;

pub struct Exporter<TStates> {
    diagram: Diagram<TStates>,
}

impl<TStates> Exporter<TStates> {
    pub fn new(diagram: Diagram<TStates>) -> Self {
        Self { diagram }
    }
}

impl<TStates> IExporter for Exporter<TStates>
where
    TStates: Copy + std::fmt::Display + PartialEq,
{
    fn export(&self) -> String {
        diagram_to_puml::export(&self.diagram)
    }

    fn get_filename(&self) -> String {
        format!("{}.puml", self.diagram.filename)
    }
}
