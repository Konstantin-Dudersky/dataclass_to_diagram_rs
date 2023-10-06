use crate::domain::models::c4_model::Diagram;

use super::{container_to_puml, context_to_puml, relation_to_puml};

pub fn export(diagram: &Diagram) -> String {
    let mut contexts = vec![];
    for context in &diagram.contexts {
        let context = context_to_puml::export_single(context.clone());
        contexts.push(context);
    }
    let mut contexts = contexts.join("\n");
    if contexts.len() > 0 {
        contexts = format!("\n{}", contexts);
    }
    format!(
        "@startuml
!include C4_Dynamic.puml{contexts}{containers}{relations}
@enduml
",
        contexts = contexts,
        containers =
            container_to_puml::export_several(diagram.containers.clone()),
        relations = relation_to_puml::export_several(diagram.relations.clone())
    )
}

// "@startuml
// !include C4_Dynamic.puml{sprites}{element_tags}{relation_tags}{contexts}{containers}{relations}{legend}
// @enduml
// "

#[cfg(test)]
#[path = "./tests/diagram_to_puml_test.rs"]
mod diagram_to_puml_test;
