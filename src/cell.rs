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

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: Value,
    pub possible_values: [Value; 9],
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            value: Value::None,
            possible_values: [
                Value::One,
                Value::Two,
                Value::Three,
                Value::Four,
                Value::Five,
                Value::Six,
                Value::Seven,
                Value::Eight,
                Value::Nine,
            ],
        }
    }

    pub fn get_print_row_values(
        &self,
        val_off1: usize,
        val_off2: usize,
        val_off3: usize,
    ) -> String {
        if self.value == Value::None {
            format!(
                "{} {} {}",
                self.possible_values[val_off1].sup_str(),
                self.possible_values[val_off2].sup_str(),
                self.possible_values[val_off3].sup_str()
            )
        } else if val_off1 == 3 {
            // print the value if on the middle row
            format!("  {}  ", self.value)
        } else {
            "     ".to_string()
        }
    }

    pub fn remove_possible_value(&mut self, val: Value) {
        self.possible_values[val.to_usize() - 1] = Value::None;
    }

    pub fn add_possible_value(&mut self, val: Value) {
        self.possible_values[val.to_usize() - 1] = val;
    }
}

impl From<Value> for Cell {
    fn from(value: Value) -> Self {
        Cell {
            value,
            possible_values: [Value::None; 9],
        }
    }
}
