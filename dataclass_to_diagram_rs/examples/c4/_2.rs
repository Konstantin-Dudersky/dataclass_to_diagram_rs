use dataclass_to_diagram::{c4_model::*, sprite, Diagrams};

pub fn create() -> Diagrams {
    let user = Context::new("Customer")
        .set_kind(ContextKind::Person)
        .set_description("People that need products")
        .set_sprite(sprite::tupadr3::FontAwesome5::users)
        .build();
    let spa = Container::new("SPA")
        .set_description("The main interface that the customer interacts with")
        .set_technology("angular")
        .set_sprite(sprite::tupadr3::Devicons::angular)
        .build();
    let api = Container::new("API")
        .set_technology("java")
        .set_description("Handles all business logic")
        .set_sprite(sprite::tupadr3::Devicons::java)
        .build();
    let db = Container::new("Database")
        .set_kind(ContainerKind::ContainerDb)
        .set_technology("Microsoft SQL")
        .set_description("Holds product, order and invoice information")
        .set_sprite(sprite::tupadr3::Devicons::msql_server)
        .build();

    let relations = vec![
        Rel::new(&user, &spa, "Uses")
            .set_technology("https")
            .build(),
        Rel::new(&spa, &api, "Uses").set_technology("https").build(),
        Rel::new(&api, &db, "Reads/Writes")
            .set_kind(RelKind::RelRight)
            .build(),
    ];

    Diagram::new("c4_2")
        .set_contexts(vec![&user])
        .set_containers(vec![&spa, &api, &db])
        .set_relations(relations)
        .set_show_legend()
        .build()
}
