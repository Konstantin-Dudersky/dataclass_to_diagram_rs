use dataclass_to_diagram::state_machine::{Diagram, Exporter};

use derive_more::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum States {
    A,
    X,
    Y,
    B,
    Z,
}

pub fn create() -> Exporter<States> {
    let mut dia = Diagram::<States>::new("state_machine_4");

    dia.add_state(States::A);
    dia.add_state(States::X).set_parent(States::A);
    dia.add_state(States::Y).set_parent(States::A);
    dia.add_state(States::B);
    dia.add_state(States::Z).set_parent(States::B);

    dia.add_transition(States::Z, States::Y);
    dia.add_transition(States::X, States::Z);

    Exporter::new(dia)
}
