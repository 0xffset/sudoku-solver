use super::value::Value;

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: Value,
    pub possible_values: [Value; 9],
    pub mutable: bool,
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
            mutable: true,
        }
    }

    /// Marks the cell as immutable.
    pub fn set_immutable(&mut self) {
        self.mutable = false;
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

    /// Returns a vector of all possible values.
    pub fn possible_values(&self) -> Vec<Value> {
        let mut values = Vec::new();

        if self.value != Value::None {
            return values;
        }

        for val in self.possible_values {
            if val != Value::None {
                values.push(val);
            }
        }
        values
    }
}
