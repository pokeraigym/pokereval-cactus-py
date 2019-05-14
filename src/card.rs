use pyo3::prelude::*;

pub const STR_RANKS: &str = "23456789TJQKA";
pub const PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
pub const INT_RANKS: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const INT_SUIT_TO_CHAR_SUIT: &str = "xshxdxxxc";
const PRETTY_REDS: &[i32] = &[2, 4];

fn CHAR_RANK_TO_INT_RANK(rank: &char) -> i32 {
    match rank {
        '2'=> 0,
        '3'=> 1,
        '4'=> 2,
        '5'=> 3,
        '6'=> 4,
        '7'=> 5,
        '8'=> 6,
        '9'=> 7,
        'T'=> 8,
        'J'=> 9,
        'Q'=> 10,
        'K'=> 11,
        'A'=> 12,
        _ => unreachable!()
    }
}

fn CHAR_SUIT_TO_INT_SUIT(suit: &char) -> i32 {
    match suit {
        's' => 1,  // spades
        'h' => 2,  // hearts
        'd' => 4,  // diamonds
        'c' => 8,  // clubs
         _ => unreachable!()
    }
}

fn PRETTY_SUITS(suit: i32) -> char {
    match suit {
        1 => '♠',
        2 => '♥',
        4 => '♦',
        8 => '♣',
         _ => unreachable!()
    }
}

#[pyclass]
pub struct Card {

}

#[pymethods]
impl Card {
    #[staticmethod]
    pub fn new(string: String) -> PyResult<i32> {
        let rank_char = string.chars().nth(0).unwrap();
        let suit_char = string.chars().nth(1).unwrap();
        let rank_int = CHAR_RANK_TO_INT_RANK(&rank_char);
        let suit_int = CHAR_SUIT_TO_INT_SUIT(&suit_char);
        let rank_prime = PRIMES[rank_int as usize];

        let bitrank = 1 << rank_int << 16;
        let suit = suit_int << 12;
        let rank = rank_int << 8;

        Ok(bitrank | suit | rank | rank_prime)
    }

    #[staticmethod]
    fn int_to_str(card_int: i32) -> PyResult<String> {
        let rank_int = Card::get_rank_int(card_int).unwrap();
        let suit_int = Card::get_suit_int(card_int).unwrap();
        Ok([STR_RANKS.chars().nth(rank_int as usize).unwrap(), INT_SUIT_TO_CHAR_SUIT.chars().nth(suit_int as usize).unwrap()].iter().collect())
    }

    #[staticmethod]
    fn get_rank_int(card_int: i32) -> PyResult<i32> {
        Ok((card_int >> 8) & 0xF)
    }

    #[staticmethod]
    fn get_suit_int(card_int: i32) -> PyResult<i32> {
        Ok((card_int >> 12) & 0xF)
    }

    #[staticmethod]
    fn get_bitrank_int(card_int: i32) -> PyResult<i32> {
        Ok((card_int >> 16) & 0x1FFF)
    }

    #[staticmethod]
    fn get_prime(card_int: i32) -> PyResult<i32> {
        Ok(card_int & 0x3F)
    }

    #[staticmethod]
    fn hand_to_binary(card_strs: Vec<String>) -> PyResult<Vec<i32>> {
        let mut ret: Vec<i32> = Vec::new();
        for c in card_strs {
            ret.push(Card::new(c).unwrap());
        }
        Ok(ret)
    }

    #[staticmethod]
    pub fn prime_product_from_hand(card_ints: Vec<i32>) -> PyResult<i32> {
        let mut product = 1;
        for c in card_ints {
            product *= c & 0xFF;
        }
        Ok(product)
    }

    #[staticmethod]
    pub fn prime_product_from_rankbits(rankbits: i32) -> PyResult<i32> {
        let mut product = 1;
        for i in INT_RANKS {
            if rankbits & (1 << i) != 0{
                product *= PRIMES[*i as usize];
            }
        }
        Ok(product)
    }

    
    #[staticmethod]
    fn int_to_pretty_str(card_int: i32) -> PyResult<String> {
        let suit_int = Card::get_suit_int(card_int).unwrap();
        let rank_int = Card::get_rank_int(card_int).unwrap();
        let s = PRETTY_SUITS(suit_int);
        let r = STR_RANKS.chars().nth(rank_int as usize).unwrap();

        Ok(format!("{}{}", r, s))
    }

    #[staticmethod]
    fn print_pretty_card(card_int: i32) -> PyResult<String> {
        Card::int_to_pretty_str(card_int)
    }

    #[staticmethod]
    fn print_pretty_cards(card_ints: Vec<i32>) -> PyResult<String> {
        let mut output = String::from(" ");

        for i in 0..card_ints.len() {
            let c = card_ints[i];
            if i != card_ints.len() - 1 {
                output.push_str(&(Card::int_to_pretty_str(c).unwrap() + ","));
            }
            else {
                output.push_str(&(Card::int_to_pretty_str(c).unwrap() + " "));
            }
        }
        Ok(output)
    }
}
