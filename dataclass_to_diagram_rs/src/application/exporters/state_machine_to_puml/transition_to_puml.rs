use crate::domain::models::state_machine::{
    State, Transition, TransitionOption,
};

pub fn export<TStates>(
    transition: &Transition<TStates>,
    begin: &State<TStates>,
    end: &State<TStates>,
) -> String {
    format!(
        "{begin} --> {end}{option}{description}",
        begin = begin.alias,
        end = end.alias,
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
        let puml = format!("{} --> {}", st1.alias, st2.alias);
        assert_eq!(export(&trans, &st1, &st2), puml);
    }

    #[test]
    fn with_description() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_description("description");

        let puml = format!("{} --> {} : description", st1.alias, st2.alias);
        assert_eq!(export(&trans, &st1, &st2), puml);
    }

    #[test]
    fn test_with_description_multiline() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_description("line 1\nline 2");

        let puml = format!("{} --> {} : line 1\\nline 2", st1.alias, st2.alias);
        assert_eq!(export(&trans, &st1, &st2), puml);
    }

    #[test]
    fn option_history() {
        let st1 = State::new(States::State1);
        let st2 = State::new(States::State2);
        let mut trans = Transition::new(States::State1, States::State2);
        trans.set_option(TransitionOption::History);

        let puml = format!("{} --> {}[H]", st1.alias, st2.alias);
        assert_eq!(export(&trans, &st1, &st2), puml);
    }
}
