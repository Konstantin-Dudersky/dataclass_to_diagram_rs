use std::rc::Rc;

use crate::domain::models::c4_model::Context;

pub fn export(context: Rc<Context>) -> String {
    let description = context.description.clone().unwrap_or_default();
    format!(
        r#"{kind}($alias = {alias}, $label = "{label}", $descr = "{description}")"#,
        kind = context.kind,
        alias = context.alias,
        label = context.label,
        description = description
    )
}

#[cfg(test)]
#[path = "./tests/context_to_puml_test.rs"]
mod context_to_puml_test;
