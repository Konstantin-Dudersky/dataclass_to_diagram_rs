use std::rc::Rc;

use handlebars::Handlebars;

use crate::{
    c4_model::{Container, Context, ElementTag, RelTag},
    domain::models::c4_model::Diagram,
    utils::clone_utils::clone_vec_of_rc,
};

use super::{
    container_to_puml, context_to_puml, relation_to_puml, tags_to_puml,
    templates::init_tamplates,
};

pub fn export(diagram: &Diagram) -> String {
    let handlebars = init_tamplates();

    let element_tags = collect_element_tags(&diagram);
    let element_tags = convert_element_tags(&handlebars, element_tags);

    let rel_tags = collect_rel_tags(diagram);
    let rel_tags = convert_rel_tags(&handlebars, rel_tags);

    let legend = match diagram.show_legend {
        true => "\n\nSHOW_LEGEND()",
        false => "",
    };
    format!(
        "@startuml
!include C4_Dynamic.puml{sprites}{element_tags}{rel_tags}{contexts}{containers}{relations}{legend}
@enduml
",
        sprites = collect_sprites(&diagram.contexts, &diagram.containers),
        element_tags = element_tags,
        contexts = context_to_puml::export_several(diagram.contexts.clone(), &handlebars),
        containers =
            container_to_puml::export_several(diagram.containers.clone(), &handlebars),
        relations = relation_to_puml::export_several(&handlebars, diagram.relations.clone()),
        legend = legend
    )
}

fn collect_element_tags(diagram: &Diagram) -> Vec<Rc<ElementTag>> {
    let mut tags_included = vec![];
    for context in &diagram.contexts {
        tags_included.extend(clone_vec_of_rc(&context.tags_included))
    }
    for container in &diagram.containers {
        tags_included.extend(clone_vec_of_rc(&container.tags_included))
    }
    tags_included
}

fn convert_element_tags(
    handlebars: &Handlebars,
    tags: Vec<Rc<ElementTag>>,
) -> String {
    let mut tags = tags
        .iter()
        .map(|t| tags_to_puml::export_element_tag(&handlebars, t))
        .collect::<Vec<String>>();
    tags.sort();
    tags.dedup();
    let mut tags = tags.join("\n");
    if tags.len() > 0 {
        tags = format!("\n\n{tags}\n");
    }
    tags
}

fn collect_rel_tags(diagram: &Diagram) -> Vec<Rc<RelTag>> {
    let mut tags = vec![];
    for rel in &diagram.relations {
        tags.extend(clone_vec_of_rc(&rel.tags))
    }
    tags
}

fn convert_rel_tags(handlebars: &Handlebars, tags: Vec<Rc<RelTag>>) -> String {
    let mut tags = tags
        .iter()
        .map(|t| tags_to_puml::export_rel_tag(&handlebars, t))
        .collect::<Vec<String>>();
    tags.sort();
    tags.dedup();
    let mut tags = tags.join("\n");
    if tags.len() > 0 {
        tags = format!("\n\n{tags}\n");
    }
    tags
}

fn collect_sprites(
    contexts: &Vec<Rc<Context>>,
    containers: &Vec<Rc<Container>>,
) -> String {
    let mut sprites = vec![];
    for context in contexts {
        let s: Vec<String> =
            context.sprite_include.iter().map(|s| s.clone()).collect();
        sprites.extend(s);
    }
    for container in containers {
        let s: Vec<String> =
            container.sprite_include.iter().map(|s| s.clone()).collect();
        sprites.extend(s);
    }
    sprites.sort();
    sprites.dedup();
    let mut sprites = sprites.join("\n");
    if sprites.len() > 0 {
        sprites = format!("\n\n{sprites}\n")
    }
    sprites
}
