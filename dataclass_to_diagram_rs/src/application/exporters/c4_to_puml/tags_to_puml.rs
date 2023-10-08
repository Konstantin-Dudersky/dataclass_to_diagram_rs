use std::{collections::HashMap, rc::Rc};

use handlebars::Handlebars;

use crate::c4_model::{ElementTag, RelTag};

pub fn export_element_tag(
    handlebar: &Handlebars,
    tag: &Rc<ElementTag>,
) -> String {
    let bg_color = tag.bg_color.clone().unwrap_or_default();
    let font_color = tag.font_color.clone().unwrap_or_default();
    let border_color = tag.border_color.clone().unwrap_or_default();
    let data = HashMap::from([
        ("tagStereo", &tag.tag_stereo),
        ("bgColor", &bg_color),
        ("fontColor", &font_color),
        ("borderColor", &border_color),
    ]);
    handlebar
        .render("element_tag", &data)
        .expect("Невозможно выполнить рендер")
}

pub fn export_rel_tag(handlebar: &Handlebars, tag: &Rc<RelTag>) -> String {
    let text_color = tag.text_color.clone().unwrap_or_default();
    let line_color = tag.line_color.clone().unwrap_or_default();
    let line_style = tag.line_style.clone().unwrap_or_default();
    let data = HashMap::from([
        ("tagStereo", &tag.tag_stereo),
        ("textColor", &text_color),
        ("lineColor", &line_color),
        ("lineStyle", &line_style),
    ]);

    handlebar
        .render("rel_tag", &data)
        .expect("Невозможно выполнить рендер")
}
