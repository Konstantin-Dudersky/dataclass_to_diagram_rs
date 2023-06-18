use std::sync::Mutex;

static ALIAS: Mutex<u32> = Mutex::new(0);

// State -----------------------------------------------------------------------

#[derive(Default)]
pub struct State {
    pub alias: u32,
    pub kind: StateKind,
    pub name: String,
    pub description: Option<String>,
    pub internal_states: Vec<State>,
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

impl State {
    pub fn new(name: &str) -> Self {
        let alias;
        {
            let mut alias_mutex = ALIAS.lock().unwrap();
            alias = *alias_mutex;
            *alias_mutex += 1;
        }
        let name = String::from(name);
        Self {
            alias,
            name,
            ..Default::default()
        }
    }

    pub fn set_description(self, description: &str) -> Self {
        Self {
            description: Some(String::from(description)),
            ..self
        }
    }

    pub fn set_kind(self, kind: StateKind) -> Self {
        Self { kind, ..self }
    }

    pub fn add_internal_state(self, state: State) -> Self {
        let mut internal_states = self.internal_states;
        internal_states.push(state);
        Self {
            internal_states,
            ..self
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias && self.name == other.name
    }
}

// Transition ------------------------------------------------------------------

pub struct Transition<'a> {
    pub begin: &'a State,
    pub end: &'a State,
    pub description: Option<String>,
    pub option: TransitionOption,
}

#[derive(Debug, Default, PartialEq)]
pub enum TransitionOption {
    #[default]
    No,
    History,
    DeepHistory,
}

impl<'a> Transition<'a> {
    pub fn new(begin: &'a State, end: &'a State) -> Self {
        Self {
            begin,
            end,
            description: None,
            option: TransitionOption::default(),
        }
    }
}

// Diagram ---------------------------------------------------------------------

#[derive(Default)]
pub struct Diagram<'a> {
    pub states: Vec<State>,
    pub transitions: Vec<Transition<'a>>,
    pub hide_empty_description: bool,
}

impl<'a> Diagram<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_state(&mut self, name: &str) -> &State {
        let new_state = State::new(name);
        self.states.push(new_state);
        self.states.last().unwrap()
    }

    pub fn add_transition(self, transition: Transition<'a>) -> Self {
        let mut transitions = self.transitions;
        transitions.push(transition);
        Self {
            transitions,
            ..self
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn diagram() {
    //     let mut dia = Diagram::new();

    //     let state1 = dia.add_state("state1");
    //     let state2 = dia.add_state("state2");

    //     let trans = Transition::new(&state1, &state2);

    //     assert_eq!(dia.states[0].name, "state1");
    //     assert_eq!(dia.hide_empty_description, true);
    // }
}
