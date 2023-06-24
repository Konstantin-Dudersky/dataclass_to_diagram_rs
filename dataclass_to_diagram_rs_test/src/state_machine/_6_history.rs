use dataclass_to_diagram::state_machine::{Diagram, Exporter, StateKind};

use derive_more::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum States {
    Start,
    State1,
    State2,
    State3,
    State3History,
    State3DeepHistory,
    State3Start,
    State3Accum,
    State3ProcessData,
    End,
}

pub fn create() -> Exporter<States> {
    let mut dia = Diagram::<States>::new("state_machine_6");

    dia.add_state(States::Start).set_kind(StateKind::Start);
    dia.add_state(States::State1);
    dia.add_state(States::State2);
    dia.add_state(States::State3);
    dia.add_state(States::End).set_kind(StateKind::End);
    dia.add_state(States::State3History)
        .set_kind(StateKind::History)
        .set_parent(States::State3);
    dia.add_state(States::State3DeepHistory)
        .set_kind(StateKind::DeepHistory)
        .set_parent(States::State3);
    dia.add_state(States::State3Start)
        .set_parent(States::State3)
        .set_kind(StateKind::Start);
    dia.add_state(States::State3Accum)
        .set_parent(States::State3)
        .set_name("Accumulate Enough Data")
        .set_description("Just a test");
    dia.add_state(States::State3ProcessData)
        .set_parent(States::State3)
        .set_name("PrecessData");

    dia.add_transition(States::Start, States::State1);
    dia.add_transition(States::State1, States::State2)
        .set_description("Succeeded");
    dia.add_transition(States::State1, States::End)
        .set_description("Aborted");
    dia.add_transition(States::State2, States::State3)
        .set_description("Succeeded");
    dia.add_transition(States::State2, States::End)
        .set_description("Aborted");
    dia.add_transition(States::State2, States::State3History)
        .set_description("Resume");
    dia.add_transition(States::State2, States::State3DeepHistory)
        .set_description("DeepResume");
    dia.add_transition(States::State3, States::End)
        .set_description("Succeeded / Save Result");
    dia.add_transition(States::State3, States::End)
        .set_description("Aborted");
    dia.add_transition(States::State3, States::State3)
        .set_description("Failed");
    dia.add_transition(States::State3Start, States::State3Accum);
    dia.add_transition(States::State3Accum, States::State3Accum)
        .set_description("New Data");
    dia.add_transition(States::State3Accum, States::State3ProcessData)
        .set_description("Enough Data");

    Exporter::new(dia)
}
