use pyo3::prelude::*;

#[pyclass]
pub struct Evaluator {
    evaluator: pokereval_cactus::evaluator::Evaluator
}

#[pymethods]
impl Evaluator {
    #[new]
    fn new_py(obj: &PyRawObject) {
        obj.init(Evaluator {
            evaluator: pokereval_cactus::evaluator::Evaluator::new()
        })
    }

    fn evaluate(&self, cards: Vec<i32>, board: Vec<i32>) -> PyResult<i32> {
        Ok(self.evaluator.evaluate(cards, board))
    }

    fn class_to_string(&self, class_int: i32) -> PyResult<String> {
        Ok(self.evaluator.class_to_string(class_int))
    }
}
