use std::cmp::PartialEq;
use std::fmt::Display;

pub struct State<TStates> {
    pub state: TStates,
    pub name: String,
    pub kind: StateKind,
    pub description: Option<String>,
    pub parent_state: Option<TStates>,
}

#[derive(Debug, Default, PartialEq)]
pub enum StateKind {
    #[default]
    General,
    Start,
    End,
    Fork,
    Join,
    Choice,
}

impl<TStates: Display> State<TStates> {
    pub fn new(state: TStates) -> Self {
        let name = format!("{}", state);
        Self {
            state,
            name,
            kind: StateKind::General,
            description: None,
            parent_state: None,
        }
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(String::from(description));
        self
    }

    pub fn set_parent(&mut self, parent_state: TStates) -> &mut Self {
        self.parent_state = Some(parent_state);
        self
    }

    pub fn set_kind(&mut self, kind: StateKind) -> &mut Self {
        self.kind = kind;
        self
    }
}

#[cfg(test)]
mod test {
    use derive_more::Display;
    use enum_iterator::Sequence;

    use super::*;

    #[test]
    fn state() {
        #[derive(Display, Sequence, PartialEq)]
        enum States {
            State1,
            State2,
        }

        let mut state = State::new(States::State1);
        state
            .set_description("state description")
            .set_kind(StateKind::End);

        assert_eq!(state.name, "State1");
        assert_eq!(state.description.unwrap(), "state description");
        assert_eq!(state.kind, StateKind::End);
    }

    #[test]
    fn parent_state() {
        #[derive(Debug, Display, Sequence, PartialEq)]
        enum States {
            State1,
            State2,
        }

        let mut state = State::new(States::State1);
        assert!(state.parent_state.is_none());

        state.set_parent(States::State2);
        assert_eq!(state.parent_state.unwrap(), States::State2);
    }
}
