use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("existing_ball")]
    BallExists,
    #[error("invalid_ball")]
    InvalidBall,
    #[error("game_finished")]
    GameFinished,
}
