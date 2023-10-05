use crate::domain::models::c4_model::Diagram;

fn export(diagram: &Diagram) -> String {
    format!(
        "@startuml
!include C4_Dynamic.puml
@enduml
"
    )
}

// "@startuml
// !include C4_Dynamic.puml{sprites}{element_tags}{relation_tags}{contexts}{containers}{relations}{legend}
// @enduml
// "

#[cfg(test)]
#[path = "./tests/diagram_to_puml_test.rs"]
mod diagram_to_puml_test;
