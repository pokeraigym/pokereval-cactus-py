use pyo3::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card::{Card, STR_RANKS};

#[pyclass]
pub struct Deck {
    cards: Vec<i32>
}

// helper method
fn shuffle() -> Vec<i32> {
    let mut deck = Deck::GetFullDeck().unwrap();
    deck.shuffle(&mut thread_rng());
    deck
}

#[pymethods]
impl Deck{
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(Deck {
            cards: shuffle(),
        })
    }

    fn shuffle(&mut self) -> PyResult<()> {
        self.cards = shuffle();
        Ok(())
    }

    fn draw(&mut self, n: i32) -> PyResult<Vec<i32>> {
        if n >= self.cards.len() as i32 {
            ()
        }
        let mut cards = Vec::new();
        for i in 0..n {
            match self.cards.pop() {
                Some(card) => cards.push(card),
                None => return Ok(cards)
            }
        }
        Ok(cards)
    }

    #[staticmethod]
    fn GetFullDeck() -> PyResult<Vec<i32>> {
        let mut deck = Vec::new();

        for rank in STR_RANKS.chars() {
            let mut spade = rank.to_string();
            spade.push('s');
            let mut heart = rank.to_string();
            heart.push('h');
            let mut diamond = rank.to_string();
            diamond.push('d');
            let mut club = rank.to_string();
            club.push('c');

            deck.push(Card::new(spade).unwrap());
            deck.push(Card::new(heart).unwrap());
            deck.push(Card::new(diamond).unwrap());
            deck.push(Card::new(club).unwrap());
        }

        Ok(deck)
    }
}