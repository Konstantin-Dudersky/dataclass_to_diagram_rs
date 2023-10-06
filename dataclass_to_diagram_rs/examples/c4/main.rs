mod _0;
mod _1;

use dataclass_to_diagram::Runner;

fn main() {
    Runner::new(vec![_0::create(), _1::create()])
}
