extern crate tombala;
use tombala::core::game::{
    add_ball,
    new_game,
};
fn main()
{
    let data = tombala::flags::handle_flags();
    let mut _state = new_game(1, data.card_index);
    // for &i in &[
    //     21, 38, 46, 76, 87, 18, 56, 63, 78, 85, 1, /* 24, */ 62, 72, 35,
    // ]
    // // for i in 1..91
    // {
    //     let w = add_ball(&mut _state, i);
    //     if let None = w
    //     {
    //         continue;
    //     }
    //     println!("{} -> {:?}", i, w);
    //     if let Some(tombala::core::types::Winning::T(_)) = w
    //     {
    //         break;
    //     }
    //     // std::thread::sleep_ms(1000);
    // }
    for i in 1..91
    {
        let w = add_ball(&mut _state, i);
        if let None = w
        {
            continue;
        }
        println!("{} -> {:?}", i, w);
        if let Some(tombala::core::types::Winning::T(_)) = w
        {
            break;
        }
        // std::thread::sleep_ms(1000);
    }
}
