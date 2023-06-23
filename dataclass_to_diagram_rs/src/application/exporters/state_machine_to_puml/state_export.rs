use std::collections::HashMap;

use crate::domain::models::state_machine::{State, StateKind};

#[derive(Debug)]
pub struct StateExport<TStates> {
    pub item_in_enum: TStates,
    pub alias: String,
    pub order: u32,
    pub name: String,
    pub kind: StateKind,
    pub description: Option<String>,
    pub internal_states: Vec<String>,
    pub internal_states_exported: Vec<InternalStateExport>,
    pub exported: Option<String>,
    pub parent_state: Option<String>,
}

impl<TStates> StateExport<TStates>
where
    TStates: Clone,
{
    pub fn from(state: &State<TStates>) -> Self {
        Self {
            item_in_enum: state.item_in_enum.clone(),
            internal_states: vec![],
            alias: state.alias.clone(),
            order: state.order,
            name: state.name.clone(),
            kind: state.kind.clone(),
            description: state.description.clone(),
            internal_states_exported: vec![],
            parent_state: state.parent_state.clone(),
            exported: None,
        }
    }

    pub fn add_internal_state(&mut self, internal_state: &str) {
        self.internal_states.push(String::from(internal_state));
    }

    pub fn add_internal_state_exported(&mut self, order: u32, exported: &str) {
        self.internal_states_exported.push(InternalStateExport {
            order,
            exported: String::from(exported),
        });
    }
}

#[derive(Debug)]
pub struct InternalStateExport {
    pub order: u32,
    pub exported: String,
}

pub fn create_state_exported<TStates>(
    states: &Vec<State<TStates>>,
) -> HashMap<String, StateExport<TStates>>
where
    TStates: Clone,
{
    let mut hash = HashMap::new();
    for state in states.iter() {
        let state_exported = StateExport::from(state);
        hash.insert(state_exported.alias.clone(), state_exported);
    }
    hash
}

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use crate::domain::models::state_machine::Diagram;

    use super::*;

    #[derive(Clone, Display, PartialEq)]
    enum States {
        State1,
        State1_1,
        State1_2,
        State1_3,
        State2,
    }

    #[test]
    fn from() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::Choice);

        let exported_state = StateExport::from(&state);

        assert_eq!(exported_state.alias, "State1");
        assert_eq!(exported_state.kind, StateKind::Choice);
    }

    #[test]
    fn create_state_exported_test() {
        let mut dia = Diagram::<States>::new("dia");
        dia.add_state(States::State1);
        dia.add_state(States::State1_1).set_parent(States::State1);
        dia.add_state(States::State1_2).set_parent(States::State1);
        dia.add_state(States::State1_3).set_parent(States::State1);
        dia.add_state(States::State2);

        let state_exported = create_state_exported(&dia.states);

        assert_eq!(state_exported.get("State1").unwrap().alias, "State1");
    }
}
