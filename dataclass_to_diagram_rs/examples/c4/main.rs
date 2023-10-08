mod _0;
mod _1;
mod _2;
mod _3;
mod _4;

use dataclass_to_diagram::Runner;

fn main() {
    Runner::new(vec![
        _0::create(),
        _1::create(),
        _2::create(),
        _3::create(),
        _4::create(),
    ])
}
