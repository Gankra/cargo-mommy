use confique::Config;
use serde::Deserialize;

pub fn load_config() -> Result<MommyConfiguration, confique::Error> {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("cargo-mommy.toml");

    let config = MommyConfiguration::builder()
        .env()
        // fallbacks to local file for testing~
        .file("./cargo-mommy.toml")
        .file(config_path)
        .load()?;

    Ok(config)
}

#[derive(Config, Debug, Deserialize)]
pub struct Responses {
    #[config(default = ["*pets your head*", "you're such a smart cookie~ ❤️", "that's a good AFFECTIONATE_TERM~ ❤️", "MOMMYS_ROLE thinks MOMMYS_PRONOUN litle AFFECTIONATE_TERM earned a big hug~ ❤️", "good AFFECTIONATE_TERM~\nMOMMYS_ROLE's so proud of you~ ❤️", "awe, what a good AFFECTIONATE_TERM~\nMOMMYS_ROLE knew you could do it~ ❤️"])]
    pub positive: Vec<String>,
    #[config(default = ["MOMMYS_ROLE believes in you~ ❤️", "do you need MOMMYS_ROLE's help~? ❤️", "MOMMYS_ROLE still loves you no matter what~ ❤️", "oh no did MOMMYS_ROLE's little AFFECTIONATE_TERM make a big mess~? ❤️", "MOMMYS_ROLE knows MOMMYS_PRONOUN little AFFECTIONATE_TERM can do better~ ❤️", "just a little further, sweetie~ ❤️"])]
    pub negative: Vec<String>,
}

#[derive(Config, Debug)]
pub struct MommyConfiguration {
    #[config(nested)]
    pub responses: Responses,
    #[config(env = "CARGO_MOMMYS_LITTLE", default = "girl")]
    pub affectionate_terms: String,
    #[config(env = "CARGO_MOMMYS_PRONOUNS", default = "her")]
    pub pronouns: String,
    #[config(env = "CARGO_MOMMYS_ROLES", default = "mommy")]
    pub roles: String,
}
