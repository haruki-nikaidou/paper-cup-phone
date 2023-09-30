use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    profile: Profile,
    database: Database,
    config: DetailedConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    #[serde(rename = "Server Name")]
    server_name: String,
    #[serde(rename = "Server Description")]
    server_description: String,
    #[serde(rename = "Admin Contact")]
    admin_contact: String,
    #[serde(rename = "Server Location")]
    server_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    #[serde(rename = "Type")]
    type_: String,
    url: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DetailedConfig {
    #[serde(rename = "Auto Delete")]
    auto_delete: bool,
    #[serde(rename = "Auto Delete Time")]
    auto_delete_time: String,
}

const PATH: &str = "./config/config.json";

pub fn load_config() -> Result<Config,String> {
    let file = std::fs::File::open(PATH).map_err(|e| e.to_string())?;
    let config: Config = serde_json::from_reader(file).map_err(|e| e.to_string())?;
    Ok(config)
}
