pub fn handle_flags()
{
    let options = clap::App::new("MugSoft Tombala")
        .arg(
            clap::Arg::with_name("gen_cards")
                .short("-g")
                .long("--gen-cards")
                .help("generate random cards"),
        )
        .get_matches();
    if options.is_present("gen_cards")
    {
        handle_g_flag();
    }
}

fn handle_g_flag()
{
    let cards = crate::core::card_generator::generate_n_cards(135);
    let _ = std::fs::write("tombala_cards.json", serde_json::to_string(&cards).unwrap());
}
