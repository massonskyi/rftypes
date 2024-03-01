use pyo3::prelude::*;
#[pyclass]
pub struct UInt8 {
    value: u8,
}

#[pymethods]
impl UInt8 {
    #[new]
    fn new(value: u8) -> Self {
        UInt8 { value }
    }

    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, new_value: u8) {
        self.value = new_value;
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}