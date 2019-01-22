use clap::{
    App,
    Arg,
};
pub fn handle_flags()
{
    let options = App::new("MugSoft Tombala")
        .arg(
            Arg::with_name("gen_cards")
                .short("-g")
                .long("--gen-cards")
                .takes_value(false)
                .help("generate random cards"),
        )
        .arg(
            Arg::with_name("cards")
                .short("-c")
                .long("--cards")
                .required_unless("gen_cards")
                .help("cards json")
                .default_value("./tombala_cards.json")
                .takes_value(true),
        )
        .get_matches();
    if options.is_present("gen_cards")
    {
        handle_g_flag();
    }
    if let Some(path) = options.value_of("cards")
    {
        handle_c_flag(path);
    }
}
fn handle_c_flag(path: &str) //{{{
{
    let cards = std::fs::read_to_string(path);
    if let Ok(cards) = cards
    {
        let cards = serde_json::from_str::<Vec<super::core::types::Card>>(&cards);
        println!("{:?}", cards.unwrap());
    }
    else if let Err(e) = cards
    {
        eprintln!("Cannot read the cards {}", e);
        std::process::exit(e.raw_os_error().unwrap_or(1));
    }
} //}}}
fn handle_g_flag() //{{{
{
    let cards = crate::core::card_generator::generate_n_cards(135);
    let _ = std::fs::write("tombala_cards.json", serde_json::to_string(&cards).unwrap());
} //}}}
