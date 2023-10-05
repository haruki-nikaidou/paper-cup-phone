use serde_derive::{Deserialize, Serialize};

const PATH: &str = "./config/config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profile: Profile,
    pub database: Database,
    pub config: DetailedConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
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
pub struct Database {
    #[serde(rename = "Type")]
    pub(crate) type_: String,
    pub(crate) url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedConfig {
    #[serde(rename = "Auto Delete")]
    pub(crate) auto_delete: bool,
    #[serde(rename = "Auto Delete Time")]
    pub(crate) auto_delete_time: String,
}

pub fn parse_config() -> Result<Config,String> {
    let file = std::fs::File::open(PATH).map_err(|e| e.to_string())?;
    let config: Config = serde_json::from_reader(file).map_err(|e| e.to_string())?;
    Ok(config)
}

pub fn time_str_to_seconds(input: &str) -> Option<u64> {
    let input = input.trim().to_lowercase();

    // Define constants
    const SECONDS_IN_DAY: u64 = 86_400;
    const SECONDS_IN_WEEK: u64 = 604_800;
    const SECONDS_IN_MONTH: u64 = 2_628_000; // Average seconds in a month (30.44 days per month)
    const SECONDS_IN_YEAR: u64 = 31_536_000; // 365 days per year

    // Check the last character to determine the unit
    let (num_str, unit) = input.split_at(input.len() - 1);
    let number: u64 = num_str.parse().ok()?;

    match unit {
        "d" | "day" | "days" => Some(number * SECONDS_IN_DAY),
        "w" | "week" | "weeks" => Some(number * SECONDS_IN_WEEK),
        "m" | "month" | "months" => Some(number * SECONDS_IN_MONTH),
        "y" | "year" | "years" => Some(number * SECONDS_IN_YEAR),
        _ => None,
    }
}