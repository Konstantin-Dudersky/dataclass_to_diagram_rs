use dataclass_to_diagram::{c4_model::*, Diagrams};

pub fn create() -> Diagrams {
    let v_1_0 = ElementTag::new("v1.0").set_border_color("#d73027").build();
    let v_1_1 = ElementTag::new("v1.1").set_font_color("#d73027").build();
    let backup = ElementTag::new("backup").set_font_color("orange").build();

    let backup_rel = RelTag::new("backup")
        .set_text_color("orange")
        .set_line_color("orange")
        .set_line_style("DashedLine()")
        .build();

    let user = Context::new("Customer")
        .set_kind(ContextKind::Person)
        .set_description("People that need products")
        .build();

    let admin = Context::new("Administrator")
        .set_kind(ContextKind::Person)
        .set_description("People that administrates the products via the new v1.1 components")
        .set_tags(vec![&v_1_1])
        .build();

    let spa = Container::new("SPA")
        .set_technology("angular")
        .set_description(
            "The main interface that the customer interacts with via v1.0",
        )
        .set_tags(vec![&v_1_0])
        .build();

    let spa_admin = Container::new("Admin SPA").set_technology("angular").set_description("The administrator interface that the customer interacts with via new v1.1").set_tags(vec![&v_1_1]).build();

    let api = Container::new("API")
        .set_technology("java")
        .set_description(
            "Handles all business logic (incl. new v1.1 extensions)",
        )
        .set_tags(vec![&v_1_0, &v_1_1])
        .build();

    let db = Container::new("Database")
        .set_kind(ContainerKind::ContainerDb)
        .set_technology("Microsoft SQL")
        .set_description("Holds product, order and invoice information")
        .build();

    let archive = Container::new("Archive")
        .set_technology("Audit logging")
        .set_description("Stores 5 years")
        .set_tags(vec![&backup])
        .build();

    let relations = vec![
        Rel::new(&user, &spa, "Uses")
            .set_technology("https")
            .build(),
        Rel::new(&spa, &api, "Uses").set_technology("https").build(),
        Rel::new(&api, &db, "Reads/Writes")
            .set_kind(RelKind::RelRight)
            .build(),
        Rel::new(&admin, &spa_admin, "Uses")
            .set_technology("https")
            .build(),
        Rel::new(&spa_admin, &api, "Uses")
            .set_technology("https")
            .build(),
        Rel::new(&api, &archive, "Writes")
            .set_kind(RelKind::RelLeft)
            .set_technology("messages")
            .set_tags(vec![&backup_rel])
            .build(),
    ];

    Diagram::new("c4_4")
        .set_contexts(vec![&user, &admin])
        .set_containers(vec![&spa, &spa_admin, &api, &db, &archive])
        .set_relations(relations)
        .set_show_legend()
        .build()
}
