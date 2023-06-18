use dataclass_to_diagram::domain::models::state_machine2::Diagram;

use derive_more::Display;

#[derive(Clone, Display, PartialEq)]
pub enum St {
    State1,
    State2,
}

#[derive(Clone, Display, PartialEq)]
pub enum Tr {
    Tr1_2,
}

pub fn create() -> Diagram<St, Tr> {
    let mut dia = Diagram::<St, Tr>::new();

    dia.add_state(St::State1).set_description("desc");

    dia.add_transition(Tr::Tr1_2, St::State1, St::State2);

    dia
}
