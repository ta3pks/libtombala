use clap::{
    App,
    Arg,
};
pub struct FlagData
{
    pub card_index: crate::core::types::CardIndex,
}
pub fn handle_flags() -> FlagData
{
    let options = App::new("MugSoft Tombala")
        //{{{ gen_cards arg
        .arg(
            Arg::with_name("gen_cards")
                .short("-g")
                .long("--gen-cards")
                .takes_value(false)
                .help("generate random cards and index them"),
        ) //}}}
        //{{{ cards arg
        .arg(
            Arg::with_name("cards")
                .short("-c")
                .long("--cards")
                .help("read indexed cards file")
                .default_value("indexed_cards.json")
                .required_unless_one(&["gen_cards", "index"])
                .takes_value(true),
        ) //}}}
        //{{{ index from cards file
        .arg(
            Arg::with_name("index")
                .help("generate index from the cards file and exit")
                .short("-i")
                .takes_value(true),
        ) //}}}
        .get_matches();
    //{{{ flag functions
    if options.is_present("gen_cards")
    {
        handle_g_flag();
    }
    if let Some(path) = options.value_of("index")
    {
        handle_i_flag(path);
    }
    else if let Some(path) = options.value_of("cards")
    {
        FlagData {
            card_index: handle_c_flag(path),
        }
    }
    else
    {
        panic!("cannot read flags");
    }
    //}}}
}
fn handle_i_flag(path: &str) -> ! //{{{
{
    let cards = std::fs::read_to_string(path).expect("cannot open cards file");
    let cards = serde_json::from_str(&cards).expect("invalid cards file format");
    let index = crate::core::game::index_cards(&cards);
    let _ = std::fs::write(
        "indexed_cards.json",
        serde_json::to_string(&index).expect("invalid cards index format"),
    );
    std::process::exit(0);
} //}}}
fn handle_c_flag(path: &str) -> crate::core::types::CardIndex //{{{
{
    let index = std::fs::read_to_string(path).expect("cannot read the cards index file");
    serde_json::from_str(&index).expect("invalid index file format")
} //}}}
fn handle_g_flag() //{{{
{
    let cards = crate::core::card_generator::generate_n_cards(135);
    let index = crate::core::game::index_cards(&cards);
    let _ = std::fs::write("cards.json", serde_json::to_string(&cards).unwrap());
    let _ = std::fs::write("indexed_cards.json", serde_json::to_string(&index).unwrap());
    std::process::exit(0);
} //}}}
