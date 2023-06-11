use super::state_machine_builders::{DiagramBuilder, StateBuilder, TransitionBuilder};

// State -----------------------------------------------------------------------

pub struct State {
    pub alias: u32,
    pub kind: StateKind,
    pub name: String,
    pub description: Option<String>,
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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(name: &str) -> StateBuilder {
        StateBuilder::new(name)
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

pub struct Diagram<'a> {
    pub states: Vec<&'a State>,
    pub transitions: Vec<&'a Transition<'a>>,
    pub hide_empty_description: bool,
}

impl<'a> Diagram<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DiagramBuilder<'a> {
        DiagramBuilder::new()
    }
}
