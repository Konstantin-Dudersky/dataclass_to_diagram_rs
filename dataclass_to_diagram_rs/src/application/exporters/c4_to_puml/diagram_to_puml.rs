use crate::domain::models::c4_model::Diagram;

use super::{container_to_puml, context_to_puml, relation_to_puml};

pub fn export(diagram: &Diagram) -> String {
    let mut sprite_include = vec![];
    for context in &diagram.contexts {
        let s: Vec<String> =
            context.sprite_include.iter().map(|s| s.clone()).collect();
        sprite_include.extend(s);
    }
    for container in &diagram.containers {
        let s: Vec<String> =
            container.sprite_include.iter().map(|s| s.clone()).collect();
        sprite_include.extend(s);
    }
    sprite_include.sort();
    sprite_include.dedup();
    let mut sprites = sprite_include.join("\n");
    if sprites.len() > 0 {
        sprites = format!("\n\n{sprites}\n")
    }

    let legend = match diagram.show_legend {
        true => "\n\nSHOW_LEGEND()",
        false => "",
    };
    format!(
        "@startuml
!include C4_Dynamic.puml{sprites}{contexts}{containers}{relations}{legend}
@enduml
",
        sprites = sprites,
        contexts = context_to_puml::export_several(diagram.contexts.clone()),
        containers =
            container_to_puml::export_several(diagram.containers.clone()),
        relations = relation_to_puml::export_several(diagram.relations.clone()),
        legend = legend
    )
}

// "@startuml
// !include C4_Dynamic.puml{sprites}{element_tags}{relation_tags}{contexts}{containers}{relations}{legend}
// @enduml
// "

#[cfg(test)]
#[path = "./tests/diagram_to_puml_test.rs"]
mod diagram_to_puml_test;
