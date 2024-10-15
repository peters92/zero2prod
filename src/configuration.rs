// logic for handling config

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize configuration reader
    let settings = config::Config::builder()
        // Add config values from a file named 'config.yaml'.
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    // Try converting into our own struct
    settings.try_deserialize::<Settings>()
}
