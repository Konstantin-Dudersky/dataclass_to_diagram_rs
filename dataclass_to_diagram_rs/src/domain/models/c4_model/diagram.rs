use std::rc::Rc;

use super::{context::System, relations::Rel};

#[derive(Clone, Default)]
pub struct Diagram {
    systems: Vec<Rc<System>>,
    relations: Vec<Rc<Rel>>,
}

impl Diagram {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_contexts(self, systems: Vec<Rc<System>>) -> Self {
        let mut new = self.clone();
        new.systems = systems;
        new
    }

    pub fn add_relations(self, relations: Vec<Rc<Rel>>) -> Self {
        let mut new = self.clone();
        new.relations = relations;
        new
    }
}
