use std::fmt::Display;

use super::{state::State, transition::Transition};

pub struct Diagram<TStates> {
    pub filename: String,
    pub states: Vec<State<TStates>>,
    pub transitions: Vec<Transition<TStates>>,
    pub hide_empty_description: bool,
}

impl<TStates> Diagram<TStates>
where
    TStates: Clone + Display + PartialEq,
{
    pub fn new(filename: &str) -> Self {
        Self {
            filename: String::from(filename),
            states: vec![],
            transitions: vec![],
            hide_empty_description: false,
        }
    }

    pub fn set_hide_empty_description(
        &mut self,
        hide_empty_description: bool,
    ) -> &mut Self {
        self.hide_empty_description = hide_empty_description;
        self
    }

    pub fn add_transition(
        &mut self,
        begin: TStates,
        end: TStates,
    ) -> &mut Transition<TStates> {
        let new_trans = Transition::new(begin, end);
        self.transitions.push(new_trans);
        self.transitions.iter_mut().last().unwrap()
    }

    pub fn add_state(&mut self, alias: TStates) -> &mut State<TStates> {
        let new_state = State::new(alias.clone());
        self.states.push(new_state);
        self.get_state_mut(alias)
    }

    pub fn get_state(&self, name: TStates) -> &State<TStates> {
        self.states
            .iter()
            .find(|state| state.item_in_enum == name)
            .expect("state in Diagram not created")
    }

    pub fn get_state_mut(&mut self, name: TStates) -> &mut State<TStates> {
        self.states
            .iter_mut()
            .find(|state| state.item_in_enum == name)
            .expect("state in Diagram not created")
    }
}

#[cfg(test)]
mod test {
    use derive_more::Display;

    use super::*;

    use super::super::{state::StateKind, transition::TransitionOption};

    #[test]
    fn test_state() {
        #[derive(Clone, Display, PartialEq)]
        enum States {
            State1,
            State2,
        }
        let mut dia1 = Diagram::<States>::new("dia1");

        dia1.add_state(States::State1)
            .set_description("desc1")
            .set_kind(StateKind::Choice);
        dia1.add_state(States::State2)
            .set_description("desc3")
            .set_description("desc4");

        dia1.add_transition(States::State1, States::State2);

        let state1 = dia1.get_state_mut(States::State1);
        assert_eq!(state1.alias, "State1");
        assert_eq!(state1.description.as_ref().unwrap(), "desc1");
        assert_eq!(state1.kind, StateKind::Choice);

        let state2 = dia1.get_state_mut(States::State2);
        assert_eq!(state2.description.as_ref().unwrap(), "desc4");
        assert_eq!(state2.kind, StateKind::General);
    }

    #[test]
    fn add_transition() {
        #[derive(Clone, Debug, Display, PartialEq)]
        enum States {
            State1,
            State2,
        }
        let mut dia1 = Diagram::<States>::new("dia");

        let trans = dia1
            .add_transition(States::State1, States::State2)
            .set_description("State1State2 description")
            .set_option(TransitionOption::History);

        assert_eq!(trans.begin, States::State1);
        assert_eq!(trans.end, States::State2);
        assert_eq!(
            trans.description.as_ref().unwrap(),
            "State1State2 description"
        );
        assert_eq!(trans.option, TransitionOption::History);
    }

    #[test]
    fn hide_empty_description() {
        #[derive(Clone, Debug, Display, PartialEq)]
        enum States {}

        let mut dia = Diagram::<States>::new("dia");

        assert_eq!(dia.hide_empty_description, false);

        dia.set_hide_empty_description(true);
        assert_eq!(dia.hide_empty_description, true);
    }
}
