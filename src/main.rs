extern crate tombala;
use tombala::command_parser::COMMAND::*;
use tombala::core::game::{
    add_ball,
    new_game,
};
fn main()
{
    #![allow(unused_must_use)]
    let data = tombala::flags::handle_flags();
    let card_index = data.card_index;
    // let card_index = Rc::new(card_index);
    let mut _state = new_game(1, &card_index);
    _state.initial = true;
    let (cmd_tx, cmd_rx) = std::sync::mpsc::sync_channel(1);
    let (msg_tx, msg_rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        tombala::delivery::socket::start_server(cmd_tx, msg_rx);
    });
    loop
    {
        let cmd = match cmd_rx.recv()
        {
            Ok(cmd) => cmd,
            Err(e) =>
            {
                println!("receive error {}", e);
                continue;
            }
        };
        match cmd
        {
            NewGame(id) =>
            {
                let id = if id.is_none() { 1 } else { id.unwrap() };
                if _state.initial
                {
                    _state = new_game(id, &card_index);
                    msg_tx.send((NewGame(Some(id)), false, Ok(None)));
                    continue;
                }
                if _state.t.len() == 0
                {
                    msg_tx.send((
                        Error(format!("last game with id {} is not complete", _state.id)),
                        false,
                        Ok(None),
                    ));
                    continue;
                };
                _state = new_game(id, &card_index);
                msg_tx.send((NewGame(Some(id)), false, Ok(None)));
            }
            NewBall(ball) =>
            {
                msg_tx.send((NewBall(ball), false, add_ball(&mut _state, ball as u8)));
            }
            Error(e) =>
            {
                msg_tx.send((Error(e), false, Ok(None)));
            }
        };
    }
}
