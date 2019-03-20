use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    sync::Arc,
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
    pub initial: bool,
    pub card_state: HashMap<u32, CardState>,
    pub card_index_by_number: Arc<CardIndex>,
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
