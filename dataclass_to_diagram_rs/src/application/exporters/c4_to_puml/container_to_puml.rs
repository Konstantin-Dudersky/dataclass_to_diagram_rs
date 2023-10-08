use std::{collections::HashMap, rc::Rc};

use handlebars::Handlebars;

use crate::domain::models::c4_model::Container;

pub fn export_single(
    container: Rc<Container>,
    handlebars: &Handlebars,
) -> String {
    let technology = container.technology.clone().unwrap_or_default();
    let description = container.description.clone().unwrap_or_default();
    let sprite = container.sprite.clone().unwrap_or_default();
    let link = container.link.clone().unwrap_or_default();
    let tags = container
        .tags
        .iter()
        .map(|t| t.tag_stereo.clone())
        .collect::<Vec<String>>()
        .join("+");
    let data = HashMap::from([
        ("kind", container.kind.to_string()),
        ("alias", container.alias.clone()),
        ("label", container.label.clone()),
        ("technology", technology),
        ("description", description),
        ("sprite", sprite),
        ("tags", tags),
        ("link", link),
    ]);
    handlebars.render("container", &data).unwrap()
}

pub fn export_several(
    containers: Vec<Rc<Container>>,
    handlebars: &Handlebars,
) -> String {
    let mut res = vec![];
    for container in &containers {
        let container = export_single(container.clone(), handlebars);
        res.push(container);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}
