use crate::domain::models::state_machine::{Transition, TransitionOption};

pub fn export(transition: &Transition) -> String {
    format!(
        "{begin} --> {end}{option}{description}",
        begin = transition.begin.alias,
        end = transition.end.alias,
        option = export_option(&transition.option),
        description = export_description(transition.description.as_deref()),
    )
}

fn export_option(option: &TransitionOption) -> String {
    match option {
        TransitionOption::No => String::from(""),
        TransitionOption::History => String::from("[H]"),
        TransitionOption::DeepHistory => String::from("[H*]"),
    }
}

fn export_description(description: Option<&str>) -> String {
    match description {
        Some(value) => format!(" : {}", value).replace("\n", "\\n"),
        None => String::from(""),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::models::state_machine::State;

//     #[test]
//     fn minimal() {
//         let st1 = State::new("begin").build();
//         let st2 = State::new("end").build();
//         let trans = Transition::new(&st1, &st2).build();
//         let puml = format!("{} --> {}", st1.alias, st2.alias);
//         assert_eq!(export(&trans), puml);
//     }

//     #[test]
//     fn with_description() {
//         let st1 = State::new("begin").build();
//         let st2 = State::new("end").build();
//         let trans = Transition::new(&st1, &st2)
//             .set_description("description")
//             .build();
//         let puml = format!("{} --> {} : description", st1.alias, st2.alias);
//         assert_eq!(export(&trans), puml);
//     }

//     #[test]
//     fn test_with_description_multiline() {
//         let st1 = State::new("begin").build();
//         let st2 = State::new("end").build();
//         let trans = Transition::new(&st1, &st2)
//             .set_description("line 1\nline 2")
//             .build();
//         let puml = format!("{} --> {} : line 1\\nline 2", st1.alias, st2.alias);
//         assert_eq!(export(&trans), puml);
//     }

//     #[test]
//     fn option_history() {
//         let st1 = State::new("begin").build();
//         let st2_1 = State::new("internal").build();
//         let st2 = State::new("end").add_internal_state(&st2_1).build();
//         let trans = Transition::new(&st1, &st2)
//             .set_option(TransitionOption::History)
//             .build();
//         let puml = format!("{} --> {}[H]", st1.alias, st2.alias);
//         assert_eq!(export(&trans), puml);
//     }
// }
