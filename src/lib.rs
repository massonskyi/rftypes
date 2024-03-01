use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString, PyTuple};
mod uint8;
mod filter;
mod functions;

use crate::uint8::UInt8;
use crate::filter::Filter;
use crate::functions::sum_as_string;
#[pymodule]
fn fastmath(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<UInt8>()?;
    m.add_class::<Filter>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/