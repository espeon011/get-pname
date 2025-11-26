use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    lang: Lang,
}

#[derive(ValueEnum, Clone)]
enum Lang {
    #[value(name = "ja")]
    Ja,
    #[value(name = "ja-Hrkt")]
    JaHrkt,
    #[value(name = "roomaji")]
    Roomaji,
    #[value(name = "ko")]
    Ko,
    #[value(name = "zh-Hant")]
    ZhHant,
    #[value(name = "zh-Hans")]
    ZhHans,
    #[value(name = "en")]
    En,
    #[value(name = "fr")]
    Fr,
    #[value(name = "de")]
    De,
    #[value(name = "es")]
    Es,
    #[value(name = "it")]
    It,
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lang_str = match self {
            Lang::Ja => "ja",
            Lang::JaHrkt => "ja-Hrkt",
            Lang::Roomaji => "roomaji",
            Lang::Ko => "ko",
            Lang::ZhHant => "zh-Hant",
            Lang::ZhHans => "zh-Hans",
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::De => "de",
            Lang::Es => "es",
            Lang::It => "it",
        };
        write!(f, "{lang_str}")
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Gets the names of all pokémon types.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let species = rustemon::pokemon::pokemon_species::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let species_names: Vec<String> = species.iter().map(|species| species.name.clone()).collect();

    for species_name in species_names {
        let pokemon_spcies =
            rustemon::pokemon::pokemon_species::get_by_name(&species_name, &rustemon_client).await;
        for name in pokemon_spcies.unwrap().names {
            if name.language.name == args.lang.to_string() {
                println!("{}", name.name);
                break;
            }
        }
    }
}
