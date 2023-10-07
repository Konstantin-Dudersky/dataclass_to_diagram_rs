use std::rc::Rc;

use crate::domain::models::c4_model::Container;

pub fn export_single(container: Rc<Container>) -> String {
    let technology = container.technology.clone().unwrap_or_default();
    let description = container.description.clone().unwrap_or_default();
    let sprite = container.sprite.clone().unwrap_or_default();
    let link = container.link.clone().unwrap_or_default();
    format!(
        r#"{kind}($alias = {alias}, $label = "{label}", $techn = "{technology}", $descr = "{description}", $sprite = "{sprite}", $link = "{link}")"#,
        kind = container.kind,
        alias = container.alias,
        label = container.label,
        technology = technology,
        description = description,
        sprite = sprite,
        link = link
    )
}

pub fn export_several(containers: Vec<Rc<Container>>) -> String {
    let mut res = vec![];
    for container in &containers {
        let container = export_single(container.clone());
        res.push(container);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}
