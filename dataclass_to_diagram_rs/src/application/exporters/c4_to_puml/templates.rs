use handlebars::Handlebars;

pub fn init_tamplates<'a>() -> Handlebars<'a> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_templates_directory(
            ".txt",
            "./dataclass_to_diagram_rs/src/templates/c4",
        )
        .unwrap();
    handlebars
}
