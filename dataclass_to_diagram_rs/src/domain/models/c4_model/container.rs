use std::rc::Rc;

use crate::utils::clone_utils::clone_vec_of_ref_rc;

use super::{
    super::super::sprite::ISprite, alias::generate_alias,
    container_kind::ContainerKind, traits::IAlias, ElementTag,
};

#[derive(Clone, Default)]
pub struct Container {
    pub alias: String,
    pub kind: ContainerKind,
    pub label: String,
    pub technology: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub sprite: Option<String>,
    pub sprite_include: Vec<String>,
    pub tags: Vec<Rc<ElementTag>>,
    pub tags_included: Vec<Rc<ElementTag>>,
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

    pub fn set_tags(self, tags: Vec<&Rc<ElementTag>>) -> Self {
        let tags = clone_vec_of_ref_rc(&tags);
        Self { tags, ..self }
    }

    pub fn build(self) -> Rc<Self> {
        let tags_included = self.tags.clone();
        Rc::new(Self {
            tags_included,
            ..self
        })
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
