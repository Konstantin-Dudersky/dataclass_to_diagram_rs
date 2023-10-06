use std::rc::Rc;

use super::{
    alias::generate_alias, container_kind::ContainerKind, traits::IAlias,
};

#[derive(Clone, Default)]
pub struct Container {
    pub alias: String,
    pub kind: ContainerKind,
    pub label: String,
    pub technology: Option<String>,
    pub description: Option<String>,
}

impl Container {
    pub fn new(label: &str) -> Self {
        Self {
            alias: generate_alias().to_string(),
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_kind(self, kind: ContainerKind) -> Self {
        Self { kind, ..self }
    }

    pub fn set_technology(self, technology: &str) -> Self {
        Self {
            technology: Some(technology.into()),
            ..self
        }
    }

    pub fn set_description(self, description: &str) -> Self {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}

impl IAlias for Container {
    fn get_alias(&self) -> String {
        self.alias.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _ = Container::new("system1").set_technology("tech").build();
        let _ = Container::new("system2").build();
    }
}
