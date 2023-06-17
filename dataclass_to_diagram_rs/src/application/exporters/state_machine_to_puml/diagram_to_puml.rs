use crate::domain::models::state_machine::{Diagram, State, Transition};

use super::{state_to_puml, transition_to_puml};

// pub fn export(diagram: &Diagram) -> String {
//     format!(
//         "@startuml{options}{states}{transitions}

// @enduml",
//         options = export_options(diagram.hide_empty_description),
//         states = export_states(&diagram.states),
//         transitions = export_transitions(&diagram.transitions),
//     )
// }

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

fn export_states(states: &Vec<&State>) -> String {
    if states.len() == 0 {
        return String::from("");
    }
    let states_str = states
        .iter()
        .map(|state| state_to_puml::export(state))
        .collect::<Vec<String>>()
        .join("\n");
    let states_str = format!("\n\n{}", states_str);
    states_str
}

fn export_transitions(transitions: &Vec<&Transition>) -> String {
    if transitions.len() == 0 {
        return String::from("");
    }
    let trans_str = transitions
        .iter()
        .map(|transition| transition_to_puml::export(transition))
        .collect::<Vec<String>>()
        .join("\n");
    let trans_str = format!("\n\n{}", trans_str);
    trans_str
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn empty() {
//         let dia = Diagram::new().build();
//         let puml = "@startuml

// @enduml";
//         assert_eq!(export(&dia), puml);
//     }

//     #[test]
//     fn hide_empty_description() {
//         let dia = Diagram::new().set_hide_empty_description(true).build();
//         let puml = "@startuml

// hide empty description

// @enduml";
//         assert_eq!(export(&dia), puml);
//     }

//     #[test]
//     fn full() {
//         // states
//         let st1 = State::new("state1").build();
//         let st21 = State::new("state21").build();
//         let st22 = State::new("state22").build();
//         let st2 = State::new("state2")
//             .add_internal_state(&st21)
//             .add_internal_state(&st22)
//             .build();
//         // transitions
//         let st1_st2 = Transition::new(&st1, &st2)
//             .set_description("line 1\nline 2")
//             .build();
//         let st21_st22 = Transition::new(&st21, &st22).build();
//         // diagram
//         let dia = Diagram::new()
//             .add_state(&st1)
//             .add_state(&st2)
//             .add_transition(&st1_st2)
//             .add_transition(&st21_st22)
//             .build();
//         let puml = format!(
//             "@startuml

// state \"state1\" as {0}
// state \"state2\" as {1} {{
//     state \"state21\" as {2}
//     state \"state22\" as {3}
// }}

// {0} --> {1} : line 1\\nline 2
// {2} --> {3}

// @enduml",
//             st1.alias, st2.alias, st21.alias, st22.alias
//         );
//         println!("{}", export(&dia));
//         assert_eq!(export(&dia), puml);
//     }
// }
