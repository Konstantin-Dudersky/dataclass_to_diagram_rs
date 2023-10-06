use std::rc::Rc;

use super::{
    alias::generate_alias, container::Container, context_kind::ContextKind,
    traits::IAlias,
};

#[derive(Clone, Default)]
pub struct Context {
    pub alias: String,
    pub kind: ContextKind,
    pub label: String,
    pub technology: Option<String>,
    pub description: Option<String>,
    pub containers: Vec<Rc<Container>>,
}

impl Context {
    pub fn new(label: &str) -> Self {
        Self {
            alias: generate_alias().to_string(),
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_kind(self, kind: ContextKind) -> Self {
        Self { kind, ..self }
    }

    pub fn set_technology(self, technology: &str) -> Self {
        let technology = Some(technology.into());
        Self { technology, ..self }
    }

    pub fn set_description(self, description: &str) -> Self {
        let description = Some(description.into());
        Self {
            description,
            ..self
        }
    }

    pub fn set_containers(self, containers: Vec<&Rc<Container>>) -> Self {
        let containers = containers.iter().map(|c| (*c).clone()).collect();
        Self { containers, ..self }
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}

impl IAlias for Context {
    fn get_alias(&self) -> String {
        self.alias.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _ = Context::new("system1").set_technology("tech").build();
        let _ = Context::new("system2").build();
    }
}
