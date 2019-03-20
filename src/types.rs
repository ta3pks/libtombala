use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::{
        Arc,
        RwLock,
    },
};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Winning
{
    C1(Vec<u32>),
    C2(Vec<u32>),
    T(Vec<u32>),
}
#[derive(Default, Debug, Clone)]
pub struct Game
{
    pub id: u64,
    pub c1: Vec<u32>,
    pub c2: Vec<u32>,
    pub t: Vec<u32>,
    pub balls: Vec<u8>,
    pub card_state: HashMap<u32, CardState>,
    pub card_index_by_number: CardIndex,
}
impl Game
{
    fn new(game_id: u64, index: CardIndex) -> Arc<RwLock<Game>>
    {
        Arc::new(RwLock::new(Game {
            id: game_id,
            card_index_by_number: index,
            ..Default::default()
        }))
    }

    fn new_game(&mut self) -> u64
    {
        self.id += 1;
        self.balls = Vec::new();
        self.c1 = Vec::new();
        self.c2 = Vec::new();
        self.t = Vec::new();
        self.card_state = HashMap::new();
        self.id
    }

    fn add_ball(&mut self, num: u8) -> Result<Option<Winning>, String>
    {
        //{{{
        if self.t.len() > 0
        {
            return Err("game finished".to_string());
        }
        else if self.balls.contains(&num)
        {
            return Err("ball exists".to_string());
        }
        self.balls.push(num);
        let val = match self.card_index_by_number.get(&num)
        {
            Some(val) => val,
            None => return Ok(None),
        };
        let mut winner_cards: HashSet<u32> = HashSet::new();
        let num_row_required_to_win = if self.c1.len() == 0
        {
            0
        }
        else if self.c2.len() == 0
        {
            1
        }
        else if self.t.len() == 0
        {
            2
        }
        else
        {
            return Ok(Some(Winning::T(self.t.clone())));
        };
        for card_info in val.iter()
        {
            let card_state = self.card_state.entry(card_info.card_id).or_default();
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
            Ok(Some(match num_row_required_to_win
            {
                0 =>
                {
                    self.c1 = winner_cards.into_iter().collect();
                    Winning::C1(self.c1.clone())
                }
                1 =>
                {
                    self.c2 = winner_cards.into_iter().collect();
                    Winning::C2(self.c2.clone())
                }
                _ =>
                {
                    self.t = winner_cards.into_iter().collect();
                    Winning::T(self.t.clone())
                }
            }))
        }
        else
        {
            Ok(None)
        }
    }

    //}}}
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
}
pub type CardIndex = HashMap<u8, Vec<Cardinfo>>; //number to card info
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Row
{
    R1,
    R2,
    R3,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cardinfo
{
    pub card_id: u32,
    pub row: Row,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CardState
{
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
}
pub type Card = (u32, [u8; 9], [u8; 9], [u8; 9]); //(id,row1,row2,row3)

pub type CmdFunc = fn(&[&str], &mut Game) -> (Option<String>, bool);