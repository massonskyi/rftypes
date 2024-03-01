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
    #[staticmethod]
    fn name() -> String {
        "UInt8".to_string()
    }
    fn __call__(&self) -> u8 {
        self.value
    }
    fn add(&self, other: &UInt8) -> UInt8 {
        UInt8 { value: self.value + other.value }
    }
    #[getter]
    fn number(&self) -> u8 {
        self.value
    }
    #[getter]
    fn real(&self) -> Self {
        // Implement real method logic
        // For demonstration, just return the same value
        Self { value: self.value }
    }
    fn sub(&self, other: &UInt8) -> UInt8 {
        UInt8 { value: self.value - other.value }
    }

    fn mul(&self, other: &UInt8) -> PyResult<UInt8> {
        Ok(UInt8 { value: self.value * other.value })
    }

    fn div(&self, other: &UInt8) -> PyResult<UInt8> {
        if other.value == 0 {
            return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division by zero"));
        }
        Ok(UInt8 { value: self.value / other.value })
    }
    #[getter]
    fn integer(&self) -> Self {
        // For demonstration, just return the same value
        Self { value: self.value }
    }
    #[getter]
    fn hex(&self) -> String {
        format!("{:X}", self.value)
    }
    #[getter]
    fn to_oct(&self) -> String {
        format!("{:o}", self.value)
    }
    #[getter]
    fn to_dec(&self) -> String {
        format!("{}", self.value)
    }
    #[getter]
    fn binary(&self) -> String {
        format!("{:b}", self.value)
    }
    #[getter]
    fn conjugate(&self) -> Self {
        // For demonstration, just return the same value
        Self { value: self.value }
    }
    #[getter]
    fn bit_length(&self) -> Self {
        Self { value: 8 }
    }
    fn repr(&self) -> String {
        // For demonstration, return a string representation
        format!("UInt8({})", self.value)
    }

    fn pow_int(&self, exponent: i32) -> Self {
        // For demonstration, raise the value to the power of the exponent
        Self { value: self.value.pow(exponent as u32) }
    }
    fn pow_uint8(&self, other: &Self) -> Self {
        // For demonstration, raise the value to the power of the other value
        Self { value: self.value.pow(other.value.into()) }
    }
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }

    fn eq(&self, other: &UInt8) -> bool {
        self.value == other.value
    }

    fn __str__(&self) -> String {
        format!("{}", self.value)
    }
}