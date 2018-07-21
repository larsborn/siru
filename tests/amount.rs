extern crate lib;

use lib::Value;
use lib::Unit;
use lib::add;

#[test]
fn it_multiplies() {
    let a: Value = Value { data: 4.5, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 9.0, unit: Unit {
        kelvin: 2,
        ..Default::default()
    }};
    assert_eq!(a * b, c)
}

#[test]
fn it_adds_compatibles() {
    let a: Value = Value { data: 4.5, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 6.5, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    assert_eq!(a + b, Ok(c))
}

#[test]
fn it_adds_incompatibles() {
    let a: Value = Value { data: 4.5, unit: Unit {
        kelvin: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        candela: 1,
        ..Default::default()
    }};
    assert_eq!(a + b, Err("Incompatible types: K != cd".to_string()))
}

#[test]
fn it_adds_and_multiplies_compatibles() {
    let a: Value = Value { data: 4.5, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        ampere: 1,
        candela: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 1.2, unit: Unit {
        ampere: 2,
        candela: 1,
        ..Default::default()
    }};
    let d: Value = Value { data: 10.2, unit: Unit {
        ampere: 2,
        candela: 1,
        ..Default::default()
    }};
    assert_eq!(a * b + c, Ok(d))
}

#[test]
fn it_adds_three_values() {
    let a: Value = Value { data: 1.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 3.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let d: Value = Value { data: 6.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    assert_eq!((a + b) + c, Ok(d))
}

#[test]
fn it_adds_three_values2() {
    let a: Value = Value { data: 1.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 3.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let d: Value = Value { data: 6.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    assert_eq!(a + (b + c), Ok(d))
}

#[test]
fn it_adds_two_pairs_of_values() {
    let a: Value = Value { data: 1.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let b: Value = Value { data: 2.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let c: Value = Value { data: 3.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let d: Value = Value { data: 6.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    let e: Value = Value { data: 12.0, unit: Unit {
        ampere: 1,
        ..Default::default()
    }};
    assert_eq!(add(a + b, c + d), Ok(e))
}
