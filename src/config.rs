use confique::Config;
use serde::Deserialize;

pub fn load_config() -> Result<MommyConfiguration, confique::Error> {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("cargo-mommy.toml");

    let config = MommyConfiguration::builder()
        .env()
        // fallback to local file for testing~
        .file("./cargo-mommy.toml")
        .file(config_path)
        .load()?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Responses {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}
impl Default for Responses {
    fn default() -> Self {
        toml::from_str(include_str!("../responses.toml")).unwrap()
    }
}

#[derive(Config, Debug)]
pub struct MommyConfiguration {
    pub responses: Option<Responses>,
    #[config(env = "CARGO_MOMMYS_LITTLE", default = "girl")]
    pub affectionate_terms: String,
    #[config(env = "CARGO_MOMMYS_PRONOUNS", default = "her")]
    pub pronouns: String,
    #[config(env = "CARGO_MOMMYS_ROLES", default = "mommy")]
    pub roles: String,
}
