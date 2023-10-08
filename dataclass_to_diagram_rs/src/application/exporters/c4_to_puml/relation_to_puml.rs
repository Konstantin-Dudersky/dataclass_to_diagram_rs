use std::{collections::HashMap, rc::Rc};

use handlebars::Handlebars;

use crate::domain::models::c4_model::Rel;

pub fn export_single(handlebars: &Handlebars, relation: Rc<Rel>) -> String {
    let technology = relation.technology.clone().unwrap_or_default();
    let link = relation.link.clone().unwrap_or_default();
    let tags = relation
        .tags
        .iter()
        .map(|t| t.tag_stereo.clone())
        .collect::<Vec<String>>()
        .join("+");
    let data = HashMap::from([
        ("kind", relation.kind.to_string()),
        ("from", relation.from.clone()),
        ("to", relation.to.clone()),
        ("label", relation.label.clone()),
        ("tags", tags),
        ("link", link),
        ("technology", technology),
    ]);
    handlebars
        .render("relation", &data)
        .expect("Невозможно выполнить рендер")
}

pub fn export_several(
    handlebars: &Handlebars,
    relations: Vec<Rc<Rel>>,
) -> String {
    let mut res = vec![];
    for rel in &relations {
        let container = export_single(handlebars, rel.clone());
        res.push(container);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}
