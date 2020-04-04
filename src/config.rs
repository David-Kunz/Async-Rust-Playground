use {async_std::fs, serde::Deserialize, serde_json};

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_port() -> u16 {
    4004
}

pub async fn get_config() -> Config {
    let config_file_content = fs::read_to_string("config")
        .await
        .unwrap_or("{}".to_string());
    let config: Config = serde_json::from_str(&config_file_content).unwrap();
    println!("Using {:?}", config);
    config
}
