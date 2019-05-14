use pyo3::prelude::*;

#[pyclass]
pub struct Deck {
    deck: pokereval_cactus::deck::Deck,
}

#[pymethods]
impl Deck {
    #[new]
    fn new_py(obj: &PyRawObject) {
        obj.init(Deck {
            deck: pokereval_cactus::deck::Deck::new()
        })
    }

    fn shuffle(&mut self) -> PyResult<()> {
        self.deck.shuffle();
        Ok(())
    }

    fn draw(&mut self, n: i32) -> PyResult<Vec<i32>> {
        Ok(self.deck.draw(n))
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    fn GetFullDeck() -> PyResult<Vec<i32>> {
        Ok(pokereval_cactus::deck::Deck::get_full_deck())
    }
}
