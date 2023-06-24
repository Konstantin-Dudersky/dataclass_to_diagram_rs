use dataclass_to_diagram::state_machine::{Diagram, Exporter, StateKind};

use derive_more::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum States {
    Start,
    Fork,
    State2,
    State3,
    Join,
    State4,
    End,
}

pub fn create() -> Exporter<States> {
    let mut dia = Diagram::<States>::new("state_machine_7");

    dia.add_state(States::Start).set_kind(StateKind::Start);
    dia.add_state(States::Fork).set_kind(StateKind::Fork);
    dia.add_state(States::State2);
    dia.add_state(States::State3);
    dia.add_state(States::Join).set_kind(StateKind::Join);
    dia.add_state(States::State4);
    dia.add_state(States::End).set_kind(StateKind::End);

    dia.add_transition(States::Start, States::Fork);
    dia.add_transition(States::Fork, States::State2);
    dia.add_transition(States::Fork, States::State3);
    dia.add_transition(States::State2, States::Join);
    dia.add_transition(States::State3, States::Join);
    dia.add_transition(States::Join, States::State4);
    dia.add_transition(States::State4, States::End);

    Exporter::new(dia)
}
