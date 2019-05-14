use pyo3::prelude::*;

mod card;
mod deck;
mod evaluator;
mod lookup;

#[pymodule]
fn pokereval_cactus_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<card::Card>().unwrap();
    m.add_class::<deck::Deck>().unwrap();
    m.add_class::<lookup::LookupTable>().unwrap();
    m.add_class::<evaluator::Evaluator>().unwrap();
    Ok(())
}
