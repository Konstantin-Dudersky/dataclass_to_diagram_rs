use dataclass_to_diagram::{c4_model::*, Diagrams};

pub fn create() -> Diagrams {
    let person = Context::new("Label")
        .set_kind(ContextKind::Person)
        .set_description("Optional Description")
        .build();
    let container = Container::new("Label")
        .set_description("Optional Description")
        .set_technology("Technology")
        .build();
    let system = Context::new("Label")
        .set_description("Optional Description")
        .build();

    Diagram::new("c4_0")
        .set_contexts(vec![&person, &system])
        .set_containers(vec![&container])
        .set_relations(vec![Rel::new(&person, &container, "Label")
            .set_technology("Optional Technology")
            .build()])
        .build()
}
