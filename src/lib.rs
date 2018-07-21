use std::fmt;
use std::ops;

#[derive(Default,PartialEq)]
pub struct Unit {
    pub ampere: i64, // A: electric current
    pub kelvin: i64, // K: temperature
    pub second: i64, // s: time
    pub metre: i64, // m: length
    pub gram: i64, // g: mass
    pub candela: i64, // cd: luminous intensity
//    mole: i64, // mol: amount of substance
}

impl fmt::Debug for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.ampere == 1 { write!(formatter, "A").expect("Could not format A"); }
        else if self.ampere != 0 { write!(formatter, "A^{}", self.ampere).expect("Could not format A"); }

        if self.kelvin == 1 { write!(formatter, "K").expect("Could not format K"); }
        else if self.kelvin != 0 { write!(formatter, "K^{}", self.kelvin).expect("Could not format K"); }

        if self.second == 1 { write!(formatter, "s").expect("Could not format s"); }
        else if self.second != 0 { write!(formatter, "s^{}", self.second).expect("Could not format s"); }

        if self.metre == 1 { write!(formatter, "m").expect("Could not format m"); }
        else if self.metre != 0 { write!(formatter, "m^{}", self.metre).expect("Could not format m"); }

        if self.gram == 1 { write!(formatter, "g").expect("Could not format g"); }
        else if self.gram != 0 { write!(formatter, "g^{}", self.gram).expect("Could not format g"); }

        if self.candela == 1 { write!(formatter, "cd").expect("Could not format cd"); }
        else if self.candela != 0 { write!(formatter, "cd^{}", self.candela).expect("Could not formatc d"); }

        return Ok(())
    }
}

impl ops::Mul<Unit> for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        Unit {
            ampere: self.ampere + rhs.ampere,
            kelvin: self.kelvin + rhs.kelvin,
            second: self.second + rhs.second,
            metre: self.metre + rhs.metre,
            gram: self.gram + rhs.gram,
            candela: self.candela + rhs.candela,
        }
    }
}

#[derive(PartialEq)]
pub struct Value {
    pub data: f64,
    pub unit: Unit,
}

pub fn add(lhs: Result<Value,String>, rhs: Result<Value,String>) -> Result<Value,String> {
    lhs.and_then(|lhs_value| lhs_value + rhs)
}

impl fmt::Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Value {{ {} {:?} }}",
            self.data,
            self.unit
        )
    }
}

impl ops::Add<Value> for Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Value) -> Result<Value, String> {
        if self.unit == rhs.unit {
            Ok(Value { data: self.data + rhs.data, unit: self.unit})
        }
        else {
            Err(format!(
                "Incompatible types: {:?} != {:?}",
                self.unit, rhs.unit
            ))
        }
    }
}

impl ops::Add<Value> for Result<Value, String> {
    type Output = Result<Value, String>;

    fn add(self, rhs: Value) -> Result<Value, String> {
        self.and_then(|self_value| self_value + rhs)
    }
}

impl ops::Add<Result<Value, String>> for Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Result<Value, String>) -> Result<Value, String> {
        rhs.and_then(|rhs_value| self + rhs_value)
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        Value { data: self.data *rhs.data, unit: self.unit * rhs.unit}
    }
}
