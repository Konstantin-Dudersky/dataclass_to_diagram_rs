use dataclass_to_diagram::ExporterRunner;

mod state_machine;

fn main() {
    ExporterRunner::new(vec![
        Box::new(state_machine::_1_simple_state::create()),
        Box::new(state_machine::_2_change_state_rendering::create()),
        Box::new(state_machine::_3_composite_state::create()),
        Box::new(state_machine::_4_substate_to_substate::create()),
        Box::new(state_machine::_5_long_name::create()),
        Box::new(state_machine::_6_history::create()),
        Box::new(state_machine::_7_fork_join::create()),
    ]);
}
