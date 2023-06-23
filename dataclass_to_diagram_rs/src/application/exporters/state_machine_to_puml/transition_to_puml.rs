use crate::domain::models::state_machine::TransitionOption;

use super::transition_export::TransitionExport;

pub fn export(transition: &TransitionExport) -> String {
    format!(
        "{begin} --> {end}{option}{description}",
        begin = transition.begin_alias,
        end = transition.end_alias,
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

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use crate::domain::models::state_machine::{State, Transition};

    use super::*;

    #[derive(Display)]
    enum States {
        State1,
        State2,
    }

    #[test]
    fn minimal() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let trans = Transition::new(States::State1, States::State2);
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);
        let puml = format!("{} --> {}", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }

    #[test]
    fn with_description() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_description("description");
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);

        let puml = format!("{} --> {} : description", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }

    #[test]
    fn test_with_description_multiline() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_description("line 1\nline 2");
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);

        let puml = format!("{} --> {} : line 1\\nline 2", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }

    #[test]
    fn option_history() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_option(TransitionOption::History);
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);

        let puml = format!("{} --> {}[H]", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }
}
