use std::collections::HashMap;
use std::fmt::Display;

use itertools::Itertools;

use crate::domain::models::state_machine::Diagram;

use super::{
    state_export::create_state_exported, state_export::StateExport,
    state_to_puml, transition_export::TransitionExport, transition_to_puml,
};

pub fn export<TStates>(diagram: &Diagram<TStates>) -> String
where
    TStates: Clone + Copy + PartialEq + Display,
{
    format!(
        "@startuml{options}{states}{transitions}

@enduml",
        options = export_options(diagram.hide_empty_description),
        states = export_states(&diagram),
        transitions = export_transitions(&diagram),
    )
}

fn export_states<TStates>(diagram: &Diagram<TStates>) -> String
where
    TStates: Clone + PartialEq + Display,
{
    if diagram.states.len() == 0 {
        return String::from("");
    }
    let mut states_exported: HashMap<String, StateExport<TStates>> =
        create_state_exported(&diagram.states);
    let parent_with_internal =
        export_states_parent_with_internal(&states_exported);
    // сохраняем инфо в структурах
    for (parent, int_states) in parent_with_internal {
        for int_state in int_states {
            states_exported
                .get_mut(&parent)
                .unwrap()
                .add_internal_state(&int_state);
        }
    }
    loop {
        // выходим, если все состояния экспортированы
        if states_exported.values().all(|se| se.exported.is_some()) {
            break;
        }
        let keys: Vec<String> = states_exported
            .keys()
            .map(|key| String::from(key))
            .collect();
        for key in keys {
            let mut state_exported = states_exported.get_mut(&key).unwrap();
            if state_exported.exported.is_some() {
                continue;
            }
            if state_exported.internal_states.len()
                != state_exported.internal_states_exported.len()
            {
                continue;
            }
            let exported = state_to_puml::export(&mut state_exported);
            match state_exported.parent_state.clone() {
                Some(value) => states_exported
                    .get_mut(&value)
                    .unwrap()
                    .add_internal_state_exported(&exported),
                None => continue,
            }
        }
    }

    let result = states_exported
        .values()
        .filter(|state| state.parent_state.is_none())
        .map(|state| String::from(state.exported.as_ref().unwrap()))
        .sorted()
        .collect::<Vec<String>>()
        .join("\n");
    format!("\n\n{}", result)
}

fn export_states_parent_with_internal<TStates>(
    states_exported: &HashMap<String, StateExport<TStates>>,
) -> HashMap<String, Vec<String>> {
    let mut int_states: HashMap<String, Vec<String>> = HashMap::new();
    for (state_alias, state_exported) in states_exported.iter() {
        let parent_state = state_exported.parent_state.clone();
        match parent_state {
            Some(value) => {
                int_states
                    .entry(value)
                    .or_default()
                    .push(String::from(state_alias));
            }
            None => continue,
        }
    }
    int_states
}

fn export_options(hide_empty_description: bool) -> String {
    let mut options: Vec<String> = vec![];
    if hide_empty_description {
        options.push(String::from("hide empty description"));
    }
    if options.len() == 0 {
        return String::from("");
    }
    let collect_str = options.join("\n");
    let collect_str = format!("\n\n{}", collect_str);
    collect_str
}

fn export_transitions<TStates>(diagram: &Diagram<TStates>) -> String
where
    TStates: Clone + Copy + PartialEq + Display,
{
    if diagram.transitions.len() == 0 {
        return String::from("");
    }
    let mut transitions_export: Vec<TransitionExport> = vec![];

    for tr in &diagram.transitions {
        let begin = diagram.get_state(tr.begin).alias.clone();
        let end = diagram.get_state(tr.end).alias.clone();
        let tr_export = TransitionExport::from(tr, &begin, &end);
        transitions_export.push(tr_export);
    }
    let result = transitions_export
        .iter()
        .map(|tr| transition_to_puml::export(tr))
        .collect::<Vec<String>>()
        .join("\n");
    format!("\n\n{}", result)
}

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use super::*;

    #[derive(Clone, Copy, Display, PartialEq)]
    enum States {
        State1,
        State2,
        State21,
        State22,
    }

    #[test]
    fn empty() {
        let dia = Diagram::<States>::new("dia");
        let puml = "@startuml

@enduml";
        assert_eq!(export(&dia), puml);
    }

    #[test]
    fn hide_empty_description() {
        let mut dia = Diagram::<States>::new("dia");
        dia.set_hide_empty_description(true);
        let puml = "@startuml

hide empty description

@enduml";
        assert_eq!(export(&dia), puml);
    }

    #[test]
    fn export_states_create_int_states_hash_test() {
        let mut dia = Diagram::<States>::new("dia");
        dia.add_state(States::State1);
        dia.add_state(States::State2);
        dia.add_state(States::State21).set_parent(States::State2);
        dia.add_state(States::State22).set_parent(States::State2);

        let state_expored = create_state_exported(&dia.states);
        let hash = export_states_parent_with_internal(&state_expored);
        assert_eq!(hash.len(), 1);
        assert!(hash.get("State1").is_none());
        assert!(hash.get("State2").is_some());
        assert!(hash
            .get("State2")
            .unwrap()
            .contains(&String::from("State21")));
        assert!(hash
            .get("State2")
            .unwrap()
            .contains(&String::from("State22")));
    }

    #[test]
    fn full() {
        // states
        let mut dia = Diagram::<States>::new("dia");
        dia.add_state(States::State1);
        dia.add_state(States::State2);
        dia.add_state(States::State21).set_parent(States::State2);
        dia.add_state(States::State22).set_parent(States::State2);

        dia.add_transition(States::State1, States::State2)
            .set_description("line 1\nline 2");
        dia.add_transition(States::State21, States::State22);

        // diagram
        let puml = format!(
            "@startuml

state \"State1\" as {0}
state \"State2\" as {1} {{
    state \"State21\" as {2}
    state \"State22\" as {3}
}}

{0} --> {1} : line 1\\nline 2
{2} --> {3}

@enduml",
            "State1", "State2", "State21", "State22",
        );
        println!("{}", export(&dia));
        assert_eq!(export(&dia), puml);
    }
}
