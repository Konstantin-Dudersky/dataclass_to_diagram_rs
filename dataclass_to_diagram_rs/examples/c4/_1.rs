use dataclass_to_diagram::{c4_model::*, Diagrams};

pub fn create() -> Diagrams {
    let admin = Context::new("Administrator")
        .set_kind(ContextKind::Person)
        .build();
    let web_app = Container::new("Web Application")
        .set_technology("C#, ASP.NET Core 2.1 MVC")
        .set_description("Allows users to compare multiple Twitter timelines")
        .build();
    let sample_system = Context::new("Sample System")
        .set_kind(ContextKind::SystemBoundary)
        .set_containers(vec![&web_app])
        .build();
    let twitter = Context::new("Twitter").build();

    let relations = vec![
        Rel::new(&admin, &web_app, "Uses")
            .set_technology("HTTPS")
            .build(),
        Rel::new(&web_app, &twitter, "Gets tweets from")
            .set_technology("HTTPS")
            .build(),
    ];

    Diagram::new("c4_1")
        .set_contexts(vec![&admin, &sample_system, &twitter])
        .set_relations(relations)
        .build()
}
