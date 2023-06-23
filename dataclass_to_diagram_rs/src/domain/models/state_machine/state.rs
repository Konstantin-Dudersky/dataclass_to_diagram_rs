use std::cmp::PartialEq;
use std::fmt::Display;

pub struct State<TStates> {
    pub item_in_enum: TStates,
    pub alias: String,
    pub name: String,
    pub kind: StateKind,
    pub description: Option<String>,
    pub parent_state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum StateKind {
    #[default]
    General,
    Start,
    End,
    Fork,
    Join,
    Choice,
}

impl<TStates> State<TStates>
where
    TStates: Display,
{
    pub fn new(item_in_enum: TStates) -> Self {
        let alias = format!("{}", item_in_enum);
        Self {
            item_in_enum,
            alias: alias.clone(),
            name: alias.clone(),
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
        self.parent_state = Some(format!("{}", parent_state));
        self
    }

    pub fn set_kind(&mut self, kind: StateKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }
}

#[cfg(test)]
mod test {
    use derive_more::Display;

    use super::*;

    #[derive(Debug, Display, PartialEq)]
    enum States {
        State1,
        State2,
    }

    #[test]
    fn state() {
        let mut state = State::new(States::State1);
        state
            .set_description("state description")
            .set_kind(StateKind::End);

        assert_eq!(state.alias, "State1");
        assert_eq!(state.description.unwrap(), "state description");
        assert_eq!(state.kind, StateKind::End);
    }

    #[test]
    fn parent_state() {
        let mut state = State::new(States::State1);
        assert!(state.parent_state.is_none());

        state.set_parent(States::State2);
        assert_eq!(state.parent_state.unwrap(), "State2");
    }
}
