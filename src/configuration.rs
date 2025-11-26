use config::{Config, File};
//Our application congigurations
#[derive(serde::Deserialize, Default)]
pub struct Settings{
    pub database: DatabaseSetting,
    pub app_port: u16,
}


#[derive(serde::Deserialize, Default)]
pub struct DatabaseSetting{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let setting = Config::builder()
            .add_source(File::with_name("configuration"))
            .build()?
            .try_deserialize::<Settings>()?;
    Ok(setting)
}