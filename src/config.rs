use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::ConfigResult;

pub(crate) fn load(path: PathBuf) -> ConfigResult<Config> {
    let file = std::fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&file)?)
}

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) targets: HashMap<String, TargetConfig>,
}

#[derive(Deserialize)]
pub(crate) struct TargetConfig {
    pub(crate) url: String,
    pub(crate) valcons_address: String,
    pub(crate) interval: u64,
    pub(crate) clients: Vec<ClientsConfig>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum ClientsConfig {
    Lcd {
        url: String,
    },
    SettenLcd {
        project_id: String,
        key: String,
        network: String,
        blockchain: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use serde_yaml::from_str;

    #[test]
    fn deserialize_from_yaml_str() {
        let yaml = indoc! {"
            targets:
              phoenix-validator:
                url: https://betteruptime.com/something
                valcons_address: terravalcons1234567890
                interval: 30
                clients:
                  - type: lcd
                    url: https://phoenix-lcd.terra.dev
                  - type: setten-lcd
                    project_id: ea08855653b64998bb47b2c03bf66de7
                    key: 02215b36969446c28b22059e63b4301b
                    network: phoenix
                    blockchain: terra
              kaiyo-validator:
                url: https://betteruptime.com/something
                valcons_address: kujivalcons1234567890
                interval: 30
                clients:
                  - type: lcd
                    url: https://lcd.kaiyo.kujira.setten.io      
        "};

        from_str::<Config>(yaml).unwrap();
    }
}
