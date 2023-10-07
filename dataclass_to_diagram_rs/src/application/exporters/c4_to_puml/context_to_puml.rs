use std::rc::Rc;

use super::{
    super::utils::increase_indent::increase_indent, container_to_puml,
};

use crate::domain::models::c4_model::{Container, Context, ContextKind};

pub fn export_single(context: Rc<Context>) -> String {
    let description = context.description.clone().unwrap_or_default();
    let containers = export_nested_containers(context.containers.clone());
    match context.kind {
        ContextKind::SystemBoundary | ContextKind::EnterpriseBoundary => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                containers = containers
            )
        }
        _ => {
            format!(
                r#"{kind}($alias = {alias}, $label = "{label}", $descr = "{description}"){containers}"#,
                kind = context.kind,
                alias = context.alias,
                label = context.label,
                description = description,
                containers = containers
            )
        }
    }
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
