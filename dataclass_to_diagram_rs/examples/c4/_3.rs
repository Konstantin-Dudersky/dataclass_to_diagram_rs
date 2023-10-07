use dataclass_to_diagram::{c4_model::*, Diagrams};

pub fn create() -> Diagrams {
    let admin = Context::new("Administrator").set_kind(ContextKind::Person).set_link("https://github.com/plantuml-stdlib/C4-PlantUML/blob/master/LayoutOptions.md#hide_person_sprite-or-show_person_spritesprite").build();

    let webapp = Container::new("Web Application")
        .set_technology("C#, ASP.NET Core 2.1 MVC")
        .set_description("Allows users to compare multiple Twitter timelines")
        .set_link("https://github.com/plantuml-stdlib/C4-PlantUML/blob/master/LayoutOptions.md")
        .build();

    let c1 = Context::new("Sample System")
        .set_kind(ContextKind::SystemBoundary).set_link("https://github.com/plantuml-stdlib/C4-PlantUML/blob/master/LayoutOptions.md").set_containers(vec![&webapp])
        .build();

    let twitter = Context::new("Twitter")
        .set_link("https://github.com/plantuml-stdlib/C4-PlantUML")
        .build();

    let relations = vec![
        Rel::new(&admin, &webapp, "Uses")
            .set_link("https://plantuml.com/link")
            .build(),
        Rel::new(&webapp, &twitter, "Gets tweets from")
            .set_technology("HTTPS")
            .set_link("https://plantuml.com/link")
            .build(),
    ];

    Diagram::new("c4_3")
        .set_contexts(vec![&admin, &c1, &twitter])
        .set_relations(relations)
        .build()
}
