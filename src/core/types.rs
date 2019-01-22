pub enum Winning
{
    C1,
    C2,
    T,
}
pub struct Game
{
    id: u64,
    c1: Vec<u8>,
    c2: Vec<u8>,
    t: Vec<u8>,
}
pub type Card = ([u8; 9], [u8; 9], [u8; 9]);
