use crate::value::Value;

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: Value,
    pub possible_values: [Value; 9],
}

impl Cell {
    /// Creates a new emmpty cell with all values being possible.
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

    /// Returns a string representation of the possible values val_off1 through val_off3. <br>
    /// ! <b>This function is used for printing the board and might be removed in the future !</b>
    pub fn __get_print_row_values(
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

    /// Removes the value from the possible values.
    pub fn remove_possible_value(&mut self, val: Value) {
        if val != Value::None {
            self.possible_values[val.to_usize() - 1] = Value::None;
        }
    }

    /// Adds the value to the possible values.
    pub fn add_possible_value(&mut self, val: Value) {
        if val != Value::None {
            self.possible_values[val.to_usize() - 1] = val;
        }
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
