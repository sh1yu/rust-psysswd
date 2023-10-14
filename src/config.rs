use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub user: UserConfig,
    pub persist: Option<PersistConfig>,
    pub remote: RemoteConfig,
    pub credentials: Vec<CredentialConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(alias = "defaultUserName")]
    pub default_user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistConfig {
    pub data_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub server_addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialConfig {
    pub user: String,
    pub token: String,
}

impl VaultConfig {
    pub async fn load_yaml(path: Option<&String>) -> Result<Self> {
        let default_config_path = "~/.pvlt/config.yaml".to_string();
        let path = path.unwrap_or(&default_config_path);
        let path = shellexpand::tilde(path).to_string();
        println!("config path: {:?}", path);
        let content = fs::read_to_string(path).await?;
        let config: VaultConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}
