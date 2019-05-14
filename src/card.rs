use pyo3::prelude::*;

#[pyclass]
pub struct Card {}

#[pymethods]
impl Card {
    #[staticmethod]
    pub fn new(string: String) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::new(string))
    }

    #[staticmethod]
    fn int_to_str(card_int: i32) -> PyResult<String> {
        Ok(pokereval_cactus::card::Card::int_to_str(card_int))
    }

    #[staticmethod]
    fn get_rank_int(card_int: i32) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::get_rank_int(card_int))
    }

    #[staticmethod]
    fn get_suit_int(card_int: i32) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::get_suit_int(card_int))
    }

    #[staticmethod]
    fn get_bitrank_int(card_int: i32) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::get_bitrank_int(card_int))
    }

    #[staticmethod]
    fn get_prime(card_int: i32) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::get_prime(card_int))
    }

    #[staticmethod]
    fn hand_to_binary(card_strs: Vec<String>) -> PyResult<Vec<i32>> {
        Ok(pokereval_cactus::card::Card::hand_to_binary(card_strs))
    }

    #[staticmethod]
    pub fn prime_product_from_hand(card_ints: Vec<i32>) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::prime_product_from_hand(card_ints))
    }

    #[staticmethod]
    pub fn prime_product_from_rankbits(rankbits: i32) -> PyResult<i32> {
        Ok(pokereval_cactus::card::Card::prime_product_from_rankbits(rankbits))
    }

    #[staticmethod]
    fn int_to_pretty_str(card_int: i32) -> PyResult<String> {
        Ok(pokereval_cactus::card::Card::int_to_pretty_str(card_int))
    }

    #[staticmethod]
    fn print_pretty_card(card_int: i32) -> PyResult<String> {
        Ok(pokereval_cactus::card::Card::int_to_pretty_str(card_int))
    }

    #[staticmethod]
    fn print_pretty_cards(card_ints: Vec<i32>) -> PyResult<String> {
        Ok(pokereval_cactus::card::Card::print_pretty_cards(card_ints))
    }
}
