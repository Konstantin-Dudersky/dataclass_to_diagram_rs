use crate::domain::models::state_machine::Transition;

pub struct TransitionExport {
    pub order: u32,
    pub begin_alias: String,
    pub end_alias: String,
    pub description: Option<String>,
}

impl TransitionExport {
    pub fn from<TStates>(
        transition: &Transition<TStates>,
        begin_alias: &str,
        end_alias: &str,
    ) -> Self {
        Self {
            order: transition.order,
            begin_alias: String::from(begin_alias),
            end_alias: String::from(end_alias),
            description: transition.description.clone(),
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
        let state1 = State::new(States::State1, 0);
        let state2 = State::new(States::State2, 1);
        let tr = Transition::new(States::State1, States::State2, 0);
        let _ = TransitionExport::from(&tr, &state1.alias, &state2.alias);
    }
}
