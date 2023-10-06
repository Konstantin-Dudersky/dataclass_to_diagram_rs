use std::rc::Rc;

use super::{
    super::diagrams::Diagrams, container::Container, context::Context,
    relations::Rel,
};

#[derive(Clone, Default)]
pub struct Diagram {
    pub name: String,
    pub contexts: Vec<Rc<Context>>,
    pub containers: Vec<Rc<Container>>,
    pub relations: Vec<Rc<Rel>>,
}

impl Diagram {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn set_contexts(self, contexts: Vec<&Rc<Context>>) -> Self {
        let mut new = self.clone();
        new.contexts = contexts.iter().map(|c| (*c).clone()).collect();
        new
    }
    pub fn set_containers(self, containers: Vec<&Rc<Container>>) -> Self {
        let mut new = self.clone();
        new.containers = containers.iter().map(|c| (*c).clone()).collect();
        new
    }

    pub fn set_relations(self, relations: Vec<Rc<Rel>>) -> Self {
        let mut new = self.clone();
        new.relations = relations;
        new
    }

    pub fn build(self) -> Diagrams {
        Diagrams::C4(self)
    }
}
