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
#[derive(Debug, Serialize, Deserialize)]
pub struct Card
{
    pub r1: [u8; 9],
    pub r2: [u8; 9],
    pub r3: [u8; 9],
}
