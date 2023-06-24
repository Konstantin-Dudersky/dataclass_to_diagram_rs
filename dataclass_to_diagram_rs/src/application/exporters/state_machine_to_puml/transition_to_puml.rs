use super::transition_export::TransitionExport;

pub fn export(transition: &TransitionExport) -> String {
    format!(
        "{begin} --> {end}{order}",
        begin = transition.begin_alias,
        end = transition.end_alias,
        order = export_description(transition.order),
    )
}

fn export_description(order: u32) -> String {
    format!(" : {}", order).replace("\n", "\\n")
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
        let st1 = State::new(States::State1, 0);
        let st2 = State::new(States::State2, 1);
        let trans = Transition::new(States::State1, States::State2, 0);
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);
        let puml = format!("{} --> {}", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }

    #[test]
    fn with_description() {
        let st1 = State::new(States::State1, 0);
        let st2 = State::new(States::State2, 1);
        let mut trans = Transition::new(States::State1, States::State2, 0);
        trans.set_description("description");
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);

        let puml = format!("{} --> {} : description", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }

    #[test]
    fn test_with_description_multiline() {
        let st1 = State::new(States::State1, 0);
        let st2 = State::new(States::State2, 1);
        let mut trans = Transition::new(States::State1, States::State2, 0);
        trans.set_description("line 1\nline 2");
        let trans_export =
            TransitionExport::from(&trans, &st1.alias, &st2.alias);

        let puml = format!("{} --> {} : line 1\\nline 2", st1.alias, st2.alias);
        assert_eq!(export(&trans_export), puml);
    }
}
