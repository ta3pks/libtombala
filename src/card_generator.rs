use super::types::Card;

use std::collections::HashSet;
fn get_unique_rand(base: u8, state: &mut HashSet<u8>) -> u8 {
    let current = base * 10 + fastrand::u8(1..10);
    if state.insert(current) {
        current
    } else {
        get_unique_rand(base, state)
    }
}
fn generate_row(prev_nums: &mut HashSet<u8>) -> [u8; 9] {
    let mut a: [u8; 9] = [0; 9];
    let mut base: u8 = 0; //row base
    let mut num_spaces = 0; //max 4 spaces are allowed in a row
    let mut num_nums = 0; //max 5 numbers
    for i in &mut a {
        if !fastrand::bool() && num_spaces < 4 {
            num_spaces += 1;
            *i = 0;
            base += 1;
            continue;
        }
        if num_nums < 5 {
            num_nums += 1;
            *i = get_unique_rand(base, prev_nums);
            base += 1;
            continue;
        }
        *i = 0;
        base += 1;
    }
    a
}
fn generate_card(id: u32) -> Card {
    let mut row_state = HashSet::new();
    (
        id,
        generate_row(&mut row_state),
        generate_row(&mut row_state),
        generate_row(&mut row_state),
    )
}
pub fn generate_cards(n: u32) -> Vec<Card> {
    let mut cards = vec![];
    for i in 0..n {
        cards.push(generate_card(i))
    }
    cards
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_row_4_zeros() {
        for _ in 1..1e5 as u32 {
            let row = generate_row(&mut HashSet::default());
            let mut zeros = 0;
            for &i in row.iter() {
                if i == 0 {
                    zeros += 1
                }
            }
            assert_eq!(zeros, 4);
        }
    }
    #[test]
    fn row_numbers_increment_by_10() {
        for _ in 1..1e5 as u32 {
            let row = generate_row(&mut HashSet::default());
            for _ in row.iter() {
                for (i, &v) in row.iter().enumerate() {
                    assert!((v as usize) > i * 10 || v == 0);
                    assert!((v as usize) < i * 10 + 10 || v == 0);
                }
            }
        }
    }
    #[test]
    fn examine_card() {
        println!("{:?}", generate_card(12));
    }
    #[test]
    fn test_generate_ncards() {
        assert_eq!(135, generate_cards(135).len())
    }
}
