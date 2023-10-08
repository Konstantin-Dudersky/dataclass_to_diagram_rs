use std::rc::Rc;

#[derive(Clone, Default)]
pub struct RelTag {
    pub tag_stereo: String,
    pub text_color: Option<String>,
    pub line_color: Option<String>,
    pub line_style: Option<String>,
    pub sprite: Option<String>,
    pub technology: Option<String>,
    pub legend_text: Option<String>,
    pub legend_sprite: Option<String>,
    pub line_thikness: Option<String>,
}

impl RelTag {
    pub fn new(tag_stereo: &str) -> Self {
        Self {
            tag_stereo: tag_stereo.into(),
            ..Default::default()
        }
    }

    pub fn set_text_color(self, text_color: &str) -> Self {
        let text_color = Some(text_color.into());
        Self { text_color, ..self }
    }

    pub fn set_line_color(self, line_color: &str) -> Self {
        let line_color = Some(line_color.into());
        Self { line_color, ..self }
    }

    pub fn set_line_style(self, line_style: &str) -> Self {
        let line_style = Some(line_style.into());
        Self { line_style, ..self }
    }

    pub fn build(self) -> Rc<Self> {
        Rc::new(self)
    }
}
