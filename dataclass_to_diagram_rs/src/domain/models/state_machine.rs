use std::sync::Mutex;

use super::state_machine_builders::TransitionBuilder;

static ALIAS: Mutex<u32> = Mutex::new(0);

// State -----------------------------------------------------------------------

#[derive(Default)]
pub struct State {
    pub alias: u32,
    pub kind: StateKind,
    pub name: String,
    pub description: Option<String>,
    pub internal_states: Option<Vec<State>>,
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
    // #[allow(clippy::new_ret_no_self)]
    // pub fn new(name: &str) -> StateBuilder {
    //     StateBuilder::new(name)
    // }
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

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(String::from(description));
        self
    }

    pub fn set_kind(&mut self, kind: StateKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn add_internal_state(&mut self, name: &str) -> &Self {
        // let int_state = State::new(name);
        // self.internal_states.push(int_state);
        // self.internal_states.last().unwrap()
        let int_state = State::new(name);
        if self.internal_states == None {
            self.internal_states = Some(vec![]);
        }
        self.internal_states.unwrap().push(int_state);
        self.internal_states.unwrap().last().unwrap()
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

#[derive(Debug, PartialEq)]
pub enum TransitionOption {
    No,
    History,
    DeepHistory,
}

impl<'a> Transition<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(begin: &'a State, end: &'a State) -> TransitionBuilder<'a> {
        TransitionBuilder::new(begin, end)
    }
}

// Diagram ---------------------------------------------------------------------

#[derive(Default)]
pub struct Diagram<'a> {
    pub states: Vec<State>,
    pub transitions: Vec<&'a Transition<'a>>,
    pub hide_empty_description: bool,
}

impl<'a> Diagram<'a> {
    // #[allow(clippy::new_ret_no_self)]
    // pub fn new() -> DiagramBuilder<'a> {
    //     DiagramBuilder::new()
    // }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    // pub fn add_state(&mut self) -> &'a State {
    //     let st = State::new("state1");
    //     self.states.push(st);
    //     &self.states.last().unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test() {
    //     let mut dia = Diagram::new();
    //     let st1 = dia.add_state();
    //     // st1.name = String::from("123");
    // }

    #[test]
    fn state() {
        let mut st1 = State::new("test state name");
        st1.set_description("test state description");
        st1.set_kind(StateKind::End);
        assert_eq!(st1.name, "test state name");
        assert_eq!(st1.description.as_ref().unwrap(), "test state description");
        assert_eq!(st1.kind, StateKind::End);
    }
}
