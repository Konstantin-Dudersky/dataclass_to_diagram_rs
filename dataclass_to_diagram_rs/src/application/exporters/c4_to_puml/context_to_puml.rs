use std::rc::Rc;

use handlebars::Handlebars;

use super::{
    super::utils::increase_indent::increase_indent, container_to_puml,
};

use crate::domain::models::c4_model::{Container, Context, ContextKind};

pub fn export_single(context: Rc<Context>, handlebars: &Handlebars) -> String {
    let description = context.description.clone().unwrap_or_default();
    let sprite = context.sprite.clone().unwrap_or_default();
    let tags = context
        .tags
        .iter()
        .map(|t| t.tag_stereo.clone())
        .collect::<Vec<String>>()
        .join("+");
    let link = context.link.clone().unwrap_or_default();
    let containers =
        export_nested_containers(context.containers.clone(), handlebars);
    match context.kind {
        ContextKind::SystemBoundary | ContextKind::EnterpriseBoundary => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}", $tags = "{tags}", $link = "{link}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                tags = tags,
                link = link,
                containers = containers
            )
        }
        _ => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}", $descr = "{description}", $sprite = "{sprite}", $tags = "{tags}", $link = "{link}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                description = description,
                sprite = sprite,
                tags = tags,
                link = link,
                containers = containers
            )
        }
    }
}

pub fn export_several(
    contexts: Vec<Rc<Context>>,
    handlebars: &Handlebars,
) -> String {
    let mut res = vec![];
    for context in &contexts {
        let context = export_single(context.clone(), handlebars);
        res.push(context);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}

fn export_nested_containers(
    containers: Vec<Rc<Container>>,
    handlebars: &Handlebars,
) -> String {
    let mut containers =
        container_to_puml::export_several(containers, handlebars);
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
