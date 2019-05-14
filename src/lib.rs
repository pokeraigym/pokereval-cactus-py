use pyo3::prelude::*;

mod deck;
mod card;
mod lookup;
mod evaluator;

#[pymodule]
fn pokereval_cactus_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<card::Card>().unwrap();
    m.add_class::<deck::Deck>().unwrap();
    m.add_class::<lookup::LookupTable>().unwrap();
    m.add_class::<evaluator::Evaluator>().unwrap();
    Ok(())
}

