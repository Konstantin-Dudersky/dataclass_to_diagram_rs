use crate::Diagrams;

pub struct ExportedDiagram {
    pub name: String,
    pub text_extension: String,
    pub content: String,
}

impl ExportedDiagram {
    pub fn new(dia: &Diagrams) -> Self {
        let (name, text_extension, content) = match dia {
            Diagrams::C4(dia) => {
                (dia.name.clone(), ".puml", super::c4_to_puml::export(dia))
            }
            Diagrams::StateMachine => todo!(),
        };
        Self {
            name,
            text_extension: text_extension.to_string(),
            content,
        }
    }
}
