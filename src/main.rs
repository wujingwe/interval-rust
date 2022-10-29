mod interval;

use crate::interval::Interval;

fn main() {
    let i1 = Interval {
        begin: 10,
        end: 20
    };
    let i2 = Interval {
        begin: 15,
        end: 23
    };
    println!("Is i1 and i2 has overlap: {}", i1.overlap(i2));
}
