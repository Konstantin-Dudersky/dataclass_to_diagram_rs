use crate::domain::models::state_machine::{Transition, TransitionOption};

pub struct TransitionExport {
    pub begin_alias: String,
    pub end_alias: String,
    pub description: Option<String>,
    pub option: TransitionOption,
}

impl TransitionExport {
    pub fn from<TStates>(
        transition: &Transition<TStates>,
        begin_alias: &str,
        end_alias: &str,
    ) -> Self {
        Self {
            begin_alias: String::from(begin_alias),
            end_alias: String::from(end_alias),
            description: transition.description.clone(),
            option: transition.option.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use crate::domain::models::state_machine::State;

    use super::*;

    #[derive(Display)]
    enum States {
        State1,
        State2,
    }

    #[test]
    fn from() {
        let state1 = State::new(States::State1);
        let state2 = State::new(States::State2);
        let tr = Transition::new(States::State1, States::State2);
        let _ = TransitionExport::from(&tr, &state1.alias, &state2.alias);
    }
}
