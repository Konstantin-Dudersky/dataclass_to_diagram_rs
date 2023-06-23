use dataclass_to_diagram::domain::models::state_machine::Diagram;

use derive_more::Display;

#[derive(Clone, Display, PartialEq)]
pub enum States {
    State1,
    State2,
}

pub fn create() -> Diagram<States> {
    let mut dia = Diagram::<States>::new();

    dia.add_state(States::State1).set_description("desc");

    dia.add_transition(States::State1, States::State2);

    dia
}
