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
    pub fn new(kind: ContainerKind, label: &str) -> Self {
        Self {
            alias: generate_alias().to_string(),
            kind,
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_technology(self, technology: &str) -> Self {
        let mut new = self.clone();
        new.technology = Some(technology.into());
        new
    }

    pub fn set_description(self, description: &str) -> Self {
        let mut new = self.clone();
        new.description = Some(description.into());
        new
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
        let s1 = Container::new(ContainerKind::Container, "system1")
            .set_technology("tech")
            .build();
        let s2 = Container::new(ContainerKind::ContainerDb, "system2").build();
    }
}
