use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};
mod errors;

pub use errors::Error;
pub type Result<T> = std::result::Result<T, errors::Error>;

#[cfg(feature = "card_generator")]
pub mod card_generator;
mod types;
pub use types::{Card, Winning};
use types::{CardIndex, Cardinfo, Row, State};
fn index_cards(cards: &[Card]) -> CardIndex {
    let mut card_index: CardIndex = Default::default();
    cards.iter().for_each(|card| {
        card.1.iter().for_each(|&num| {
            if num == 0 {
                return;
            }
            let line = card_index.0.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R1,
            })
        });
        card.2.iter().for_each(|&num| {
            if num == 0 {
                return;
            }
            let line = card_index.0.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R2,
            })
        });
        card.3.iter().for_each(|&num| {
            if num == 0 {
                return;
            }
            let line = card_index.0.entry(num).or_default();
            line.push(Cardinfo {
                card_id: card.0,
                row: Row::R3,
            })
        });
    });
    card_index
}
pub trait TombalaGameState {
    fn new_session(&mut self);
    fn balls(&self) -> Vec<u8>;
    fn c1(&self) -> Vec<u32>;
    fn c2(&self) -> Vec<u32>;
    fn t(&self) -> Vec<u32>;
    fn set_c1(&mut self, c1: &[u32]);
    fn set_c2(&mut self, c2: &[u32]);
    fn set_t(&mut self, t: &[u32]);
    fn push_ball(&mut self, num: u8) -> u8;
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TombalaGame<T: TombalaGameState> {
    state: T,
    card_index: CardIndex,
    game_state: State,
}
impl<T: TombalaGameState> Deref for TombalaGame<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl<T: TombalaGameState> DerefMut for TombalaGame<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<T: TombalaGameState> TombalaGame<T> {
    pub fn new(state: T, cards: &[Card]) -> Self {
        let mut s = Self {
            state,
            card_index: index_cards(cards),
            game_state: Default::default(),
        };
        s.resume_previous_game();
        s
    }
    fn resume_previous_game(&mut self) {
        let balls = self.balls().to_vec();
        for ball in balls {
            self.new_ball_helper(ball, true).ok();
        }
    }
    //if is_restore is true, it will not push the ball to the balls vector just will update internal state
    fn new_ball_helper(&mut self, num: u8, is_restore: bool) -> Result<Option<Winning>> {
        use errors::Error::*;
        if !self.t().is_empty() {
            return Err(GameFinished);
        } else if self.balls().contains(&num) {
            return Err(BallExists);
        }
        //actual game not restoring previous game
        if !is_restore {
            self.push_ball(num);
        }
        let val = match self.card_index.0.get(&num) {
            Some(val) => val,
            None => return Ok(None),
        };
        let mut winner_cards: HashSet<u32> = HashSet::new();
        let num_row_required_to_win = if self.c1().is_empty() {
            0
        } else if self.c2().is_empty() {
            1
        } else if self.t().is_empty() {
            2
        } else {
            return Ok(Some(Winning::T(self.t().to_owned())));
        };
        for card_info in val.iter() {
            let card_state = self.game_state.0.entry(card_info.card_id).or_default();
            let mut num_rows_completed = 0;
            match card_info.row {
                Row::R1 => card_state.r1 += 1,
                Row::R2 => card_state.r2 += 1,
                Row::R3 => card_state.r3 += 1,
            };
            if card_state.r1 > 4 {
                num_rows_completed += 1;
            }
            if card_state.r2 > 4 {
                num_rows_completed += 1;
            }
            if card_state.r3 > 4 {
                num_rows_completed += 1;
            }
            if num_rows_completed > num_row_required_to_win {
                winner_cards.insert(card_info.card_id);
            }
        }
        if !winner_cards.is_empty() {
            Ok(Some(match num_row_required_to_win {
                0 => {
                    let winners = winner_cards.into_iter().collect::<Vec<_>>();
                    self.set_c1(&winners);
                    Winning::C1(winners)
                }
                1 => {
                    let winners = winner_cards.into_iter().collect::<Vec<_>>();
                    self.set_c2(&winners);
                    Winning::C2(winners)
                }
                _ => {
                    let winners = winner_cards.into_iter().collect::<Vec<_>>();
                    self.set_t(&winners);
                    Winning::T(winners)
                }
            }))
        } else {
            Ok(None)
        }
    }
    pub fn new_ball(&mut self, num: u8) -> Result<Option<Winning>> {
        self.new_ball_helper(num, false)
    }
    pub fn new_session(&mut self) {
        self.state.new_session();
        self.game_state = Default::default();
    }
}
