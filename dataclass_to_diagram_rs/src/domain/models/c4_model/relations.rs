use std::rc::Rc;

use super::context::System;

#[derive(Clone, Default)]
pub struct Rel {
    from: Rc<System>,
    to: Rc<System>,
}

impl Rel {
    pub fn new(from: &Rc<System>, to: &Rc<System>) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            ..Default::default()
        }
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
        let s1 = System::new("s1").build();
        let s2 = System::new("s2").build();

        let r1 = Rel::new(&s1, &s2).build();
        let r2 = Rel::new(&s1, &s2).build();
    }
}
