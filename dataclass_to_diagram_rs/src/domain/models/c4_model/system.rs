use std::rc::Rc;

use super::alias::generate_alias;

#[derive(Clone, Default)]
pub struct System {
    pub alias: String,
    pub label: String,
    pub technology: Option<String>,
}

impl System {
    pub fn new(label: &str) -> Self {
        Self {
            alias: generate_alias().to_string(),
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_technology(self, technology: &str) -> Self {
        let mut new = self.clone();
        new.technology = Some(technology.into());
        new
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s1 = System::new("system1").set_technology("tech").build();
        let s2 = System::new("system2").build();
    }
}
