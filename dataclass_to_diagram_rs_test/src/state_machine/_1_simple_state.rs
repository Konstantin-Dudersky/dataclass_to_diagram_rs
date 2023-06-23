use dataclass_to_diagram::state_machine::{Diagram, Exporter, StateKind};

use derive_more::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum States {
    Start,
    State1,
    State2,
    End,
}

pub fn create() -> Exporter<States> {
    let mut dia = Diagram::<States>::new("state_machine_1");

    dia.add_state(States::Start).set_kind(StateKind::Start);
    dia.add_state(States::State1)
        .set_description("this is string\nthis is another string");
    dia.add_state(States::State2);
    dia.add_state(States::End).set_kind(StateKind::End);

    dia.add_transition(States::Start, States::State1);
    dia.add_transition(States::State1, States::State2);
    dia.add_transition(States::State1, States::End);
    dia.add_transition(States::State2, States::End);

    Exporter::new(dia)
}
