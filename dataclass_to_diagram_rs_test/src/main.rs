use dataclass_to_diagram::domain::models::state_machine::State;

fn main() {
    let state1 = State::new("test").build();
    let state2 = State::new("test").build();
    println!("{}", state2.alias);
}
