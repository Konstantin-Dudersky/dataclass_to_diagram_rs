use std::sync::Mutex;

use super::state_machine::{Diagram, State, StateKind, Transition, TransitionOption};

static ALIAS: Mutex<u32> = Mutex::new(0);

// State -----------------------------------------------------------------------
#[derive(Default)]
pub struct StateBuilder<'a> {
    alias: u32,
    kind: StateKind,
    name: &'a str,
    description: Option<&'a str>,
}

impl<'a> StateBuilder<'a> {
    pub fn new(name: &'a str) -> Self {
        let alias;
        {
            let mut alias_mutex = ALIAS.lock().unwrap();
            alias = *alias_mutex;
            *alias_mutex += 1;
        }
        Self {
            alias,
            name,
            ..Default::default()
        }
    }

    pub fn set_kind(self, kind: StateKind) -> Self {
        Self { kind, ..self }
    }

    pub fn set_description(self, description: &'a str) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn build(self) -> State {
        State {
            alias: self.alias,
            kind: self.kind,
            name: String::from(self.name),
            description: self.description.map(String::from),
        }
    }
}

// Transition ------------------------------------------------------------------

pub struct TransitionBuilder<'a> {
    begin: &'a State,
    end: &'a State,
    description: Option<&'a str>,
    option: TransitionOption,
}

impl<'a> TransitionBuilder<'a> {
    pub fn new(begin: &'a State, end: &'a State) -> Self {
        Self {
            begin,
            end,
            description: None,
            option: TransitionOption::No,
        }
    }

    pub fn set_description(self, description: &'a str) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn set_option(self, option: TransitionOption) -> Self {
        Self { option, ..self }
    }

    pub fn build(self) -> Transition<'a> {
        Transition {
            begin: self.begin,
            end: self.end,
            description: self.description.map(String::from),
            option: self.option,
        }
    }
}

// Diagram ---------------------------------------------------------------------

#[derive(Default)]
pub struct DiagramBuilder<'a> {
    states: Vec<&'a State>,
    transitions: Vec<&'a Transition<'a>>,
}

impl<'a> DiagramBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_state(self, state: &'a State) -> Self {
        let mut states = self.states;
        states.push(state);
        Self { states, ..self }
    }

    pub fn add_transition(self, transition: &'a Transition) -> Self {
        let mut transitions = self.transitions;
        transitions.push(transition);
        Self {
            transitions,
            ..self
        }
    }

    pub fn build(self) -> Diagram<'a> {
        Diagram {
            states: self.states,
            transitions: self.transitions,
            hide_empty_description: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let state1 = State::new("test state name")
            .set_kind(StateKind::End)
            .set_description("state description")
            .build();
        assert_eq!(state1.name, "test state name");
        assert_eq!(state1.description.unwrap(), "state description");
        assert_eq!(state1.kind, StateKind::End);
    }

    #[test]
    fn test_transition() {
        let state1 = State::new("state1").build();
        let state2 = State::new("state2").build();
        let trans = Transition::new(&state1, &state2)
            .set_description("transition description")
            .set_option(TransitionOption::DeepHistory)
            .build();

        assert_eq!(trans.begin.name, "state1");
        assert_eq!(trans.end.name, "state2");
        assert_eq!(trans.description.unwrap(), "transition description");
        assert_eq!(trans.option, TransitionOption::DeepHistory);
    }

    #[test]
    fn test_diagram() {
        let state1 = State::new("state1").build();
        let state2 = State::new("state2").build();
        let trans1 = Transition::new(&state1, &state2).build();
        let diagram = Diagram::new()
            .add_state(&state1)
            .add_state(&state2)
            .add_transition(&trans1);

        assert!(diagram.states.contains(&&state1));
        assert!(diagram.states.contains(&&state2));
    }
}
