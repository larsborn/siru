#[macro_use]
extern crate nom;
extern crate lib;

use nom::digit;
use std::str::FromStr;
use lib::Value;
use lib::Unit;

fn from_digit(input: &str) -> Result<f64, std::num::ParseFloatError> {
   FromStr::from_str(input)
}

fn from_unit(input: &str) -> Result<Unit, String> {
    match input {
        "A" => Ok(Unit {ampere: 1, ..Default::default()}),
        "K" => Ok(Unit {kelvin: 1, ..Default::default()}),
        "s" => Ok(Unit {second: 1, ..Default::default()}),
        "m" => Ok(Unit {metre: 1, ..Default::default()}),
        "g" => Ok(Unit {gram: 1, ..Default::default()}),
        "cd" => Ok(Unit {candela: 1, ..Default::default()}),
        _ => Ok(Unit {..Default::default()}),
    }
}

fn from_prefix(input: &str) -> Result<f64, String> {
    match input {
        "k" => Ok(1000.0),
        "m" => Ok(0.001),
        "n" => Ok(0.000001),
        "u" => Ok(0.000000001),
        "M" => Ok(1000000.0),
        "G" => Ok(1000000000.0),
        "T" => Ok(1000000000000.0),
        "c" => Ok(0.01),
        "d" => Ok(0.1),
        _ => Ok(1.0)
    }
}

named!(number<&str, f64>,
    map_res!(
        recognize!(
            tuple!(
                digit,
                opt!(
                    pair!(
                        tag!("."),
                        digit
                    )
                )
            )
        ),
        from_digit
    )
);

named!(unit<&str, Unit>,
    map_res!(
        recognize!(
            alt!(
                tag!("A") | tag!("K") | tag!("s") | tag!("m") | tag!("g") | tag!("cd")
            )
        ),
        from_unit
    )
);


named!(prefix<&str, f64>,
    map_res!(
        recognize!(
            alt!(
                tag!("k") | tag!("m") | tag!("n") | tag!("u")
                | tag!("M") | tag!("G") | tag!("T")
                | tag!("c") | tag!("d")
            )
        ),
        from_prefix
    )
);

named!(myvalue<&str, Value>,
    do_parse!(
        coef: number >>
        prefix: opt!(prefix) >>
        unit: unit >>
        (Value { data: coef * prefix.unwrap_or(1.0), unit: unit })
    )
);

fn main() {
    println!("{:?}", myvalue("12ccd"));
    println!("{:?}", myvalue("12.345Tg"));

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
