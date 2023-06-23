use dataclass_to_diagram::ExporterRunner;

mod state_machine;

fn main() {
    ExporterRunner::new(vec![
        Box::new(state_machine::_1_simple_state::create()),
        Box::new(state_machine::_2_change_state_rendering::create()),
        Box::new(state_machine::_3_composite_state::create()),
    ]);
}
