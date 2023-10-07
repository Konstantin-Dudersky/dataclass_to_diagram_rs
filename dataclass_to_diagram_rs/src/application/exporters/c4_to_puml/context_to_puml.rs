use std::rc::Rc;

use super::{
    super::utils::increase_indent::increase_indent, container_to_puml,
};

use crate::domain::models::c4_model::{Container, Context, ContextKind};

pub fn export_single(context: Rc<Context>) -> String {
    let description = context.description.clone().unwrap_or_default();
    let sprite = context.sprite.clone().unwrap_or_default();
    let link = context.link.clone().unwrap_or_default();
    let containers = export_nested_containers(context.containers.clone());
    match context.kind {
        ContextKind::SystemBoundary | ContextKind::EnterpriseBoundary => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}", $link = "{link}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                link = link,
                containers = containers
            )
        }
        _ => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}", $descr = "{description}", $sprite = "{sprite}", $link = "{link}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                description = description,
                sprite = sprite,
                link = link,
                containers = containers
            )
        }
    }
}

pub fn export_several(contexts: Vec<Rc<Context>>) -> String {
    let mut res = vec![];
    for context in &contexts {
        let context = export_single(context.clone());
        res.push(context);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}

fn export_nested_containers(containers: Vec<Rc<Container>>) -> String {
    let mut containers = container_to_puml::export_several(containers);
    if containers.len() > 0 {
        containers = increase_indent(&containers);
        containers = format!(
            r#"{{{containers}
}}
"#,
            containers = containers
        )
    };
    containers
}

#[cfg(test)]
#[path = "./tests/context_to_puml_test.rs"]
mod context_to_puml_test;
