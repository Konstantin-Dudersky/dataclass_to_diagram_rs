use std::rc::Rc;

use crate::domain::models::c4_model::Container;

pub fn export_single(container: Rc<Container>) -> String {
    let description = container.description.clone().unwrap_or_default();
    format!(
        r#"{kind}($alias = {alias}, $label = "{label}", $descr = "{description}")"#,
        kind = container.kind,
        alias = container.alias,
        label = container.label,
        description = description
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
