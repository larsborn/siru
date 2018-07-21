extern crate lib;

use lib::Value;
use lib::Unit;

fn main() {
    let a: Value = Value { data: 4.5, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        candela: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 1.2, unit: Unit {
        candela: 1,
        ampere: 1,
        ..Default::default()
    }};
    println!("{:?}", a * b + c);
}
