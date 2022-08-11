use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::One => write!(f, "1"),
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::None => write!(f, " "),
        }
    }
}

impl Value {
    /// Returns the value as super script representation.
    pub fn sup_str(&self) -> &'static str {
        match self {
            Value::One => "¹",
            Value::Two => "²",
            Value::Three => "³",
            Value::Four => "⁴",
            Value::Five => "⁵",
            Value::Six => "⁶",
            Value::Seven => "⁷",
            Value::Eight => "⁸",
            Value::Nine => "⁹",
            Value::None => " ",
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Value::One => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::None => 0,
        }
    }
}

impl<S: AsRef<str>> From<S> for Value {
    fn from(val: S) -> Self {
        let val = val.as_ref();
        match val {
            "1" => Value::One,
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            _ => Value::None,
        }
    }
}
