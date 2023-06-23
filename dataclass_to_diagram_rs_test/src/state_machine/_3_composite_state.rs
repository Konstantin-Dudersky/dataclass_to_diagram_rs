use dataclass_to_diagram::state_machine::{Diagram, Exporter, StateKind};

use derive_more::Display;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum States {
    Start,
    NotShooting,
    NotShootingStart,
    NotShootingIdle,
    NotShootingConfiguring,
    NotShootingConfiguringStart,
    NotShootingConfiguringNewValueSelection,
    NotShootingConfiguringNewValuePreview,
    NotShootingConfiguringNewValuePreviewState1,
    NotShootingConfiguringNewValuePreviewState2,
}

pub fn create() -> Exporter<States> {
    let mut dia = Diagram::<States>::new("state_machine_3");

    dia.add_state(States::Start).set_kind(StateKind::Start);
    dia.add_state(States::NotShooting).set_name("NotShooting");

    dia.add_state(States::NotShootingStart)
        .set_parent(States::NotShooting)
        .set_kind(StateKind::Start);
    dia.add_state(States::NotShootingIdle)
        .set_parent(States::NotShooting)
        .set_name("Idle");
    dia.add_state(States::NotShootingConfiguring)
        .set_parent(States::NotShooting)
        .set_name("Configuring");
    dia.add_state(States::NotShootingConfiguringStart)
        .set_parent(States::NotShootingConfiguring)
        .set_kind(StateKind::Start);
    dia.add_state(States::NotShootingConfiguringNewValueSelection)
        .set_name("NewValueSelection")
        .set_parent(States::NotShootingConfiguring);
    dia.add_state(States::NotShootingConfiguringNewValuePreview)
        .set_name("NewValuePreview")
        .set_parent(States::NotShootingConfiguring);
    dia.add_state(States::NotShootingConfiguringNewValuePreviewState1)
        .set_name("State1")
        .set_parent(States::NotShootingConfiguringNewValuePreview);
    dia.add_state(States::NotShootingConfiguringNewValuePreviewState2)
        .set_name("State2")
        .set_parent(States::NotShootingConfiguringNewValuePreview);

    dia.add_transition(States::Start, States::NotShooting);
    dia.add_transition(States::NotShootingStart, States::NotShootingIdle);
    dia.add_transition(States::NotShootingIdle, States::NotShootingConfiguring)
        .set_description("EvConfig");
    dia.add_transition(States::NotShootingConfiguring, States::NotShootingIdle)
        .set_description("EvConfig");
    dia.add_transition(
        States::NotShootingConfiguringStart,
        States::NotShootingConfiguringNewValueSelection,
    );
    dia.add_transition(
        States::NotShootingConfiguringNewValueSelection,
        States::NotShootingConfiguringNewValuePreview,
    )
    .set_description("EvNewValue");
    dia.add_transition(
        States::NotShootingConfiguringNewValuePreview,
        States::NotShootingConfiguringNewValueSelection,
    )
    .set_description("EvNewValueRejected");
    dia.add_transition(
        States::NotShootingConfiguringNewValuePreview,
        States::NotShootingConfiguringNewValueSelection,
    )
    .set_description("EvNewValueSaved");
    dia.add_transition(
        States::NotShootingConfiguringNewValuePreviewState1,
        States::NotShootingConfiguringNewValuePreviewState2,
    )
    .set_description("EvNewValueSaved");

    Exporter::new(dia)
}
