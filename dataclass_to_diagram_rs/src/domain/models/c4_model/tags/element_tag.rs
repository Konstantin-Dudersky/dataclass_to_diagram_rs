use std::rc::Rc;

#[derive(Clone, Default)]
pub struct ElementTag {
    pub tag_stereo: String,
    pub bg_color: Option<String>,
    pub font_color: Option<String>,
    pub border_color: Option<String>,
    pub shadowing: Option<String>,
    pub shape: Option<String>,
    pub sprite: Option<String>,
    pub technology: Option<String>,
    pub legend_text: Option<String>,
    pub legend_sprite: Option<String>,
    pub border_style: Option<String>,
    pub border_thikness: Option<String>,
}

impl ElementTag {
    pub fn new(tag_stereo: &str) -> Self {
        Self {
            tag_stereo: tag_stereo.into(),
            ..Default::default()
        }
    }

    pub fn set_font_color(self, font_color: &str) -> Self {
        Self {
            font_color: Some(font_color.into()),
            ..self
        }
    }

    pub fn set_border_color(self, border_color: &str) -> Self {
        Self {
            border_color: Some(border_color.into()),
            ..self
        }
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}
