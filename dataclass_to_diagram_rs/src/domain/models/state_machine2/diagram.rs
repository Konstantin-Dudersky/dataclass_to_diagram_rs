use std::fmt::Display;

use super::{state::State, transition::Transition};

pub struct Diagram<TStates, TTransitions> {
    pub states: Vec<State<TStates>>,
    pub transitions: Vec<Transition<TTransitions, TStates>>,
}

impl<TStates, TTransitions> Diagram<TStates, TTransitions>
where
    TStates: Clone + Display + PartialEq,
    TTransitions: Clone + PartialEq,
{
    pub fn new() -> Self {
        Self {
            states: vec![],
            transitions: vec![],
        }
    }

    pub fn add_transition(
        &mut self,
        alias: TTransitions,
        begin: TStates,
        end: TStates,
    ) -> &mut Transition<TTransitions, TStates> {
        let new_trans = Transition::new(alias.clone(), begin, end);
        self.transitions.push(new_trans);
        self.get_transition(alias)
    }

    pub fn add_state(&mut self, alias: TStates) -> &mut State<TStates> {
        let new_state = State::new(alias.clone());
        self.states.push(new_state);
        self.get_state(alias)
    }

    pub fn get_state(&mut self, name: TStates) -> &mut State<TStates> {
        self.states
            .iter_mut()
            .find(|state| state.state == name)
            .expect("state in Diagram not created")
    }

    pub fn get_transition(
        &mut self,
        alias: TTransitions,
    ) -> &mut Transition<TTransitions, TStates> {
        self.transitions
            .iter_mut()
            .find(|trans| trans.alias == alias)
            .expect("transition not created")
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
        #[derive(Display, PartialEq, Clone)]
        enum Trans {
            State1State2,
        }

        let mut dia1 = Diagram::<States, Trans>::new();

        dia1.add_state(States::State1)
            .set_description("desc1")
            .set_kind(StateKind::Choice);
        dia1.add_state(States::State2)
            .set_description("desc3")
            .set_description("desc4");

        dia1.add_transition(
            Trans::State1State2,
            States::State1,
            States::State2,
        );
        println!("{:?}", dia1.get_state(States::State1).name);
    }

    #[test]
    fn add_transition() {
        #[derive(Clone, Debug, Display, PartialEq)]
        enum States {
            State1,
            State2,
        }
        #[derive(Display, PartialEq, Clone)]
        enum Trans {
            State1State2,
        }
        let mut dia1 = Diagram::<States, Trans>::new();

        dia1.add_transition(
            Trans::State1State2,
            States::State1,
            States::State2,
        )
        .set_description("State1State2 description")
        .set_option(TransitionOption::History);

        assert_eq!(
            dia1.get_transition(Trans::State1State2).begin,
            States::State1
        );
        assert_eq!(
            dia1.get_transition(Trans::State1State2).end,
            States::State2
        );
        assert_eq!(
            dia1.get_transition(Trans::State1State2)
                .description
                .as_ref()
                .unwrap(),
            "State1State2 description"
        );
        assert_eq!(
            dia1.get_transition(Trans::State1State2).option,
            TransitionOption::History
        );
    }
}
