use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Winning {
    C1(Vec<u32>),
    C2(Vec<u32>),
    T(Vec<u32>),
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct CardIndex(pub(crate) HashMap<u8, Vec<Cardinfo>>); //number to card info

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum Row {
    R1,
    R2,
    R3,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Cardinfo {
    pub card_id: u32,
    pub row: Row,
}
#[derive(Default, Serialize, Deserialize, Clone)]
pub(crate) struct State(pub(crate) HashMap<u32, SingleCardState>); //card id to card state
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SingleCardState {
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
}
pub type Card = (u32, [u8; 9], [u8; 9], [u8; 9]); //(id,row1,row2,row3)
