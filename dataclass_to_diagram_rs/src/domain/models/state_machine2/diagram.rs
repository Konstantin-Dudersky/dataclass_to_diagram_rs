use std::fmt::Display;

use enum_iterator::{all, Sequence};

use super::state::State;

struct Diagram<TStates, TTransitions> {
    states: Vec<State<TStates>>,
    transitions: Vec<Tra>,
}

impl<TStates: Display + PartialEq + Sequence, TTransitions>
    Diagram<TStates, TTransitions>
{
    pub fn new() -> Self {
        let mut states = vec![];
        for state in all::<TStates>().collect::<Vec<_>>() {
            states.push(State::new(state));
        }
        Self { states }
    }

    pub fn state(&mut self, name: TStates) -> &mut State<TStates> {
        self.states
            .iter_mut()
            .find(|state| state.state == name)
            .expect("state in Diagram not created")
    }
}
#[cfg(test)]
mod test {
    use derive_more::Display;

    use super::*;

    #[test]
    fn test_state() {
        #[derive(Display, Sequence, PartialEq)]
        enum States {
            State1,
            State2,
        }
        let mut dia1 = Diagram::<States>::new();

        dia1.state(States::State1)
            .set_description("desc1")
            .set_description("desc2");
        dia1.state(States::State2)
            .set_description("desc3")
            .set_description("desc4");
        println!("{:?}", dia1.state(States::State1).name);
    }
}
