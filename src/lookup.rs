use pyo3::prelude::*;

use std::collections::HashMap;

#[pyclass]
pub struct LookupTable {
    table: pokereval_cactus::lookup::LookupTable,
}

#[pymethods]
impl LookupTable {
    #[new]
    fn new_py(obj: &PyRawObject) {
        obj.init(
            LookupTable {
                table: pokereval_cactus::lookup::LookupTable::new(),
            }
        )
    }

    #[getter]
    fn flush_lookup(&self) -> PyResult<HashMap<i32, i32>> {
        Ok(self.table.flush_lookup.clone())
    }

    #[getter]
    fn unsuited_lookup(&self) -> PyResult<HashMap<i32, i32>> {
        Ok(self.table.unsuited_lookup.clone())
    }
}
