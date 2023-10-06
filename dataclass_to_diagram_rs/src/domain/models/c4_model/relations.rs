use std::rc::Rc;

use super::{relation_kind::RelKind, traits::IAlias};

#[derive(Clone, Default)]
pub struct Rel {
    pub kind: RelKind,
    pub from: String,
    pub to: String,
    pub label: String,
    pub technology: Option<String>,
}

impl Rel {
    pub fn new<TFrom, TTo>(from: &Rc<TFrom>, to: &Rc<TTo>, label: &str) -> Self
    where
        TFrom: IAlias,
        TTo: IAlias,
    {
        Self {
            from: from.get_alias(),
            to: to.get_alias(),
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn set_kind(self, kind: RelKind) -> Self {
        let mut new = self.clone();
        new.kind = kind;
        new
    }

    pub fn set_technology(self, technology: &str) -> Self {
        Self {
            technology: Some(technology.into()),
            ..self
        }
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{context::Context, context_kind::ContextKind},
        *,
    };

    #[test]
    fn test1() {
        let s1 = Context::new(ContextKind::System, "s1").build();
        let s2 = Context::new(ContextKind::System, "s2").build();

        let r1 = Rel::new(&s1, &s2, "rel 1").build();
        let r2 = Rel::new(&s1, &s2, "rel 2").build();
    }
}
