use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub paths: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeneralConfig {
    pub listen_addr: String,
    pub tls_key: Option<String>,
    pub tls_password: Option<String>,
}