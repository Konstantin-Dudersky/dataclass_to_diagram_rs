use std::collections::HashMap;
use std::fmt::Display;

use crate::domain::models::state_machine::{Diagram, State, Transition};

use super::{state_exported::StateExported, state_to_puml, transition_to_puml};

/*
Проходим по всем состояниям диаграммы, создаем HashMap

Проходим по всем состояниям диаграммы и заносим в internal_states алиасы вложенных состояний.

Запускаем цикл по HashMap. Цикл выполняется, пока у всех состояний в поле export не будет значения.
Если список internal_states пустой - делаем экспорт.
Если список internal_states не пустой - проверяем все вложенные состояния, если у всех есть значение export, собираем все экспортированные значения в internal_states_exported и экспортируем данное состояние.

*/

fn export_states<TStates>(diagram: &Diagram<TStates>) -> String
where
    TStates: Clone + PartialEq + Display,
{
    let mut states_exported: HashMap<String, StateExported> = HashMap::new();

    // инициализируем HashMap
    for state in &diagram.states {
        states_exported.insert(
            String::from(&state.alias),
            StateExported::new(&state.alias, state.parent_state.is_some()),
        );
    }

    // заполняем внутренние состояния по parent_state
    for state in &diagram.states {
        match &state.parent_state {
            Some(parent_state) => {
                let se = states_exported
                    .get_mut(parent_state)
                    .expect("parent_state not found");
                se.add_internal_state(&state.alias);
            }
            None => continue,
        }
    }

    loop {
        for state_exported in states_exported.values_mut() {
            if state_exported.internal_states.len() == 0 {
                let state = diagram.get_state(state_exported.alias);
                state_exported.export = state_to_puml(diagram.get)
            }
        }
        break;
    }
    // if states.len() == 0 {
    //     return String::from("");
    // }
    // let states_str = states
    //     .iter()
    //     .map(|state| state_to_puml::export(state))
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // let states_str = format!("\n\n{}", states_str);
    // states_str

    String::from("")
}

pub fn export<TStates>(diagram: &Diagram<TStates>) -> String {
    format!(
        "@startuml{options}{states}{transitions}

@enduml",
        options = export_options(diagram.hide_empty_description),
        states = export_states(&diagram),
        // transitions = export_transitions(&diagram.transitions),
        transitions = "",
    )
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

// fn export_transitions<TStates>(
//     transitions: &Vec<&Transition<TStates>>,
// ) -> String {
//     if transitions.len() == 0 {
//         return String::from("");
//     }
//     let trans_str = transitions
//         .iter()
//         .map(|transition| transition_to_puml::export(transition, ))
//         .collect::<Vec<String>>()
//         .join("\n");
//     let trans_str = format!("\n\n{}", trans_str);
//     trans_str<TStates><TStates>
// }

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use super::*;

    #[derive(Clone, Display, PartialEq)]
    enum States {
        State1,
        State2,
        State21,
        State22,
    }

    #[test]
    fn empty() {
        let dia = Diagram::<States>::new();
        let puml = "@startuml

@enduml";
        assert_eq!(export(&dia), puml);
    }

    #[test]
    fn hide_empty_description() {
        let mut dia = Diagram::<States>::new();
        dia.set_hide_empty_description(true);
        let puml = "@startuml

hide empty description

@enduml";
        assert_eq!(export(&dia), puml);
    }

    #[test]
    fn full() {
        // states
        let mut dia = Diagram::<States>::new();
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

state \"state1\" as {0}
state \"state2\" as {1} {{
    state \"state21\" as {2}
    state \"state22\" as {3}
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
