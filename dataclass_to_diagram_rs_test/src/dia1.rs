use dataclass_to_diagram::domain::models::state_machine::Diagram;

use derive_more::Display;

#[derive(Clone, Display, PartialEq)]
pub enum St {
    State1,
    State2,
}

pub fn create() -> Diagram<St> {
    let mut dia = Diagram::<St>::new();

    dia.add_state(St::State1).set_description("desc");

    dia.add_transition(St::State1, St::State2);

    dia
}
