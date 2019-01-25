use super::types::{
    Card,
    CardIndex,
    Cardinfo,
    Game,
    Row,
    Winning,
};
use std::collections::{
    HashMap,
    HashSet,
};
/// add a new ball if not already added and report back the winning status
pub fn new_game(game_id: u64, index: CardIndex) -> Game
{
    Game {
        id: game_id,
        card_index_by_number: index,
        ..Default::default()
    }
}
pub fn add_ball(state: &mut Game, num: u8) -> Option<Winning>
{
    state.balls.push(num);
    let val = match state.card_index_by_number.get_mut(&num)
    {
        Some(val) => val,
        None => return None,
    };
    let mut winner_cards: HashSet<u32> = HashSet::new();
    let num_row_required_to_win = if state.c1.len() == 0
    {
        0
    }
    else if state.c2.len() == 0
    {
        1
    }
    else if state.t.len() == 0
    {
        2
    }
    else
    {
        return Some(Winning::T(state.t.clone()));
    };
    for card_info in val.iter()
    {
        let card_state = state.card_state.entry(card_info.card_id).or_default();
        let mut num_rows_completed = 0;
        match card_info.row
        {
            Row::R1 => card_state.r1 += 1,
            Row::R2 => card_state.r2 += 1,
            Row::R3 => card_state.r3 += 1,
        };
        if card_state.r1 > 4
        {
            num_rows_completed += 1;
        }
        if card_state.r2 > 4
        {
            num_rows_completed += 1;
        }
        if card_state.r3 > 4
        {
            num_rows_completed += 1;
        }
        if num_rows_completed > num_row_required_to_win
        {
            winner_cards.insert(card_info.card_id);
        }
    }
    if !winner_cards.is_empty()
    {
        Some(match num_row_required_to_win
        {
            0 =>
            {
                state.c1 = winner_cards.into_iter().collect();
                Winning::C1(state.c1.clone())
            }
            1 =>
            {
                state.c2 = winner_cards.into_iter().collect();
                Winning::C2(state.c2.clone())
            }
            _ =>
            {
                state.t = winner_cards.into_iter().collect();
                Winning::T(state.t.clone())
            }
        })
    }
    else
    {
        None
    }
}
#[cfg(test)] //{{{
mod tests
{
    use super::*;
    #[test]
    fn test_new_ball()
    {
        let mut state = new_game(12, vec![]);
        new_ball(&mut state, 15);
        assert_eq!(state.balls.len(), 1);
        assert_eq!(state.balls[0], 15);
        new_ball(&mut state, 16);
        assert_eq!(state.balls.len(), 2);
        assert_eq!(state.balls[1], 16);
    }
} //}}}
pub fn index_cards(cards: &Vec<Card>) -> CardIndex //{{{
{
    let mut card_index: CardIndex = HashMap::new();
    cards.iter().for_each(|card| {
        //{{{ row 1
        card.1.iter().for_each(|&num| {
            if num == 0
            {
                return;
            }
            let line = card_index.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R1,
            })
        }); //}}}
            //{{{ row 2
        card.2.iter().for_each(|&num| {
            if num == 0
            {
                return;
            }
            let line = card_index.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R2,
            })
        }); //}}}
            //{{{ row 3
        card.3.iter().for_each(|&num| {
            if num == 0
            {
                return;
            }
            let line = card_index.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R3,
            })
        }); //}}}
    });
    card_index
} //}}}
