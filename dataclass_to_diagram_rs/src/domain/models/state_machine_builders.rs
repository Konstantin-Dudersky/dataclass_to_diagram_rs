use std::sync::Mutex;

use super::state_machine::{
    Diagram, State, StateKind, Transition, TransitionOption,
};

static ALIAS: Mutex<u32> = Mutex::new(0);

// State -----------------------------------------------------------------------
#[derive(Default)]
pub struct StateBuilder<'a> {
    alias: u32,
    kind: StateKind,
    name: &'a str,
    description: Option<&'a str>,
    internal_states: Option<Vec<&'a State>>,
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

    pub fn add_internal_state(self, state: &'a State) -> Self {
        let mut internal_states = self.internal_states.unwrap_or_default();
        internal_states.push(state);
        Self {
            internal_states: Some(internal_states),
            ..self
        }
    }

    // pub fn build(self) -> State {
    //     State {
    //         alias: self.alias,
    //         kind: self.kind,
    //         name: String::from(self.name),
    //         description: self.description.map(String::from),
    //         internal_states: self.internal_states,
    //     }
    // }
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
    hide_empty_description: bool,
}

impl<'a> DiagramBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_hide_empty_description(
        self,
        hide_empty_description: bool,
    ) -> Self {
        Self {
            hide_empty_description,
            ..self
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

    // pub fn build(self) -> Diagram<'a> {
    //     Diagram {
    //         states: self.states,
    //         transitions: self.transitions,
    //         hide_empty_description: self.hide_empty_description,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn internal_states() {
    //         let int_state1 = State::new("internal 1").build();
    //         let int_state2 = State::new("internal 2").build();
    //         let state1 = State::new("state with internal states")
    //             .add_internal_state(&int_state1)
    //             .add_internal_state(&int_state2)
    //             .build();

    //         let internal_states = state1.internal_states.unwrap();
    //         assert_eq!(internal_states[0].name, "internal 1");
    //         assert_eq!(internal_states[1].name, "internal 2");
    //     }

    //     #[test]
    //     fn test_transition() {
    //         let state1 = State::new("state1").build();
    //         let state2 = State::new("state2").build();
    //         let trans = Transition::new(&state1, &state2)
    //             .set_description("transition description")
    //             .set_option(TransitionOption::DeepHistory)
    //             .build();

    //         assert_eq!(trans.begin.name, "state1");
    //         assert_eq!(trans.end.name, "state2");
    //         assert_eq!(trans.description.unwrap(), "transition description");
    //         assert_eq!(trans.option, TransitionOption::DeepHistory);
    //     }

    //     #[test]
    //     fn test_diagram() {
    //         let state1 = State::new("state1").build();
    //         let state2 = State::new("state2").build();
    //         let trans1 = Transition::new(&state1, &state2).build();
    //         let diagram = Diagram::new()
    //             .add_state(&state1)
    //             .add_state(&state2)
    //             .add_transition(&trans1);

    //         assert!(diagram.states.contains(&&state1));
    //         assert!(diagram.states.contains(&&state2));
    //     }
}