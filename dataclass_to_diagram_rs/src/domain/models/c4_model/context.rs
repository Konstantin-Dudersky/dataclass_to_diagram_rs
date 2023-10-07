use std::rc::Rc;

use super::{
    super::super::sprite::ISprite, alias::generate_alias, container::Container,
    context_kind::ContextKind, traits::IAlias,
};

#[derive(Clone, Default)]
pub struct Context {
    pub alias: String,
    pub kind: ContextKind,
    pub label: String,
    pub technology: Option<String>,
    pub description: Option<String>,
    pub containers: Vec<Rc<Container>>,
    pub sprite: Option<String>,
    pub sprite_include: Vec<String>,
    pub link: Option<String>,
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

    pub fn set_sprite<TSprite>(self, sprite: TSprite) -> Self
    where
        TSprite: ISprite,
    {
        let (sprite, sprite_include) = sprite.export();
        let sprite = Some(sprite);
        Self {
            sprite,
            sprite_include,
            ..self
        }
    }

    pub fn set_link(self, link: &str) -> Self {
        Self {
            link: Some(link.into()),
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
