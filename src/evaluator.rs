use itertools::Itertools;
use pyo3::prelude::*;
use crate::card::Card;
use crate::deck::Deck;
use crate::lookup::{LookupTable, MAX_HIGH_CARD, RANK_CLASS_TO_STRING};

#[pyclass]
pub struct Evaluator {
    table: LookupTable,
}

impl Evaluator {
    fn five(&self, cards: Vec<i32>) -> i32 {
        if  (cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000) != 0 {
            let handOR = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16;
            let prime = Card::prime_product_from_rankbits(handOR).unwrap();
            self.table.flush_lookup[&prime]
        }
        else {
            let prime = Card::prime_product_from_hand(cards).unwrap();
            self.table.unsuited_lookup[&prime]
        }
    }

    fn six(&self, cards: Vec<i32>) -> i32 {
        let mut minimum = MAX_HIGH_CARD;

        let all5cardcombos = cards.iter().combinations(5);

        for combo in all5cardcombos {
            let score = self.five(vec![*combo[0], *combo[1], *combo[2], *combo[3], *combo[4]]);
            if score < minimum {
                minimum = score;
            }
        }

        minimum
    }

    fn seven(&self, cards: Vec<i32>) -> i32 {
        self.six(cards)
    }
}

#[pymethods]
impl Evaluator {
     #[new]
    fn new(obj: &PyRawObject) { 
        obj.init(Evaluator {
            table: LookupTable::new()
        })
    }

    fn evaluate(&self, mut cards: Vec<i32>, mut board: Vec<i32>) -> PyResult<i32> {
        cards.append(&mut board);
        let length = cards.len();
        let value = match length {
            5 => self.five(cards),
            6 => self.six(cards),
            7 => self.seven(cards),
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn class_to_string(&self, class_int: i32) -> PyResult<String> {
        Ok(RANK_CLASS_TO_STRING(class_int))
    }
}