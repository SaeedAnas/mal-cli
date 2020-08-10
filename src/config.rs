use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdin, Write},
    path::{Path, PathBuf},
};

const DEFAULT_PORT: u16 = 7878;
const DEFAULT_USER_AGENT: &str = "mal-cli";
const CONFIG_FILE: &str = "mal.yml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "mal-cli";
const TOKEN_CACHE_FILE: &str = ".mal_token_cache.json";

#[derive(Debug)]
pub enum ConfigError {
    /// Represents an invalid config file
    EmptyConfig,
    /// Represents a failure to read from input
    ReadError,
    /// Represents a nonexistent path error
    PathError,
    /// Represents a serde_yaml parse error
    ParseError(serde_yaml::Error),
    /// Represents all other failures
    IOError(std::io::Error),
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigError::EmptyConfig => None,
            ConfigError::ReadError => None,
            ConfigError::PathError => None,
            ConfigError::ParseError(_) => None,
            ConfigError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConfigError::EmptyConfig => write!(f, "Source contains no data"),
            ConfigError::ReadError => write!(f, "Could not read file"),
            ConfigError::PathError => write!(f, "Path not found"),
            ConfigError::ParseError(ref err) => err.fmt(f),
            ConfigError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IOError(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub client_id: String,
    pub user_agent: Option<String>,
    pub port: Option<u16>,
}

pub struct ConfigPaths {
    pub config_file_path: PathBuf,
    pub auth_cache_path: PathBuf,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let paths = Self::get_paths()?;
        if paths.config_file_path.exists() {
            let config_string = fs::read_to_string(&paths.config_file_path)?;
            let config_yml: AppConfig = serde_yaml::from_str(&config_string)?;

            Ok(config_yml)
        } else {
            println!(
                "Config will be saved to {}",
                paths.config_file_path.display()
            );

            println!("\nHow to get setup:\n");

            let instructions = [
                "Go to the myanimelist api page - https://myanimelist.net",
                "Click `Create ID` and create an app",
                &format!(
                    "Add `http://localhost:{}/callback` to the Redirect URIs",
                    DEFAULT_PORT
                ),
                "You are now ready to authenticate with myanimelist!",
            ];

            let mut number = 1;
            for item in instructions.iter() {
                println!("   {}. {}", number, item);
                number += 1;
            }

            let mut client_id = String::new();
            println!("\nEnter your client ID: ");
            stdin().read_line(&mut client_id)?;
            let client_id = client_id.trim().to_string();

            let mut user_agent = String::new();
            println!("\nEnter User Agent (default {}): ", DEFAULT_USER_AGENT);
            stdin().read_line(&mut user_agent)?;
            let user_agent = match user_agent.trim().len() {
                0 => DEFAULT_USER_AGENT.to_string(),
                _ => user_agent,
            };

            let mut port = String::new();
            println!("\nEnter port of redirect uri (default {}): ", DEFAULT_PORT);
            stdin().read_line(&mut port)?;
            let port = port.trim().parse::<u16>().unwrap_or(DEFAULT_PORT);

            let config_yml = AppConfig {
                client_id,
                user_agent: Some(user_agent),
                port: Some(port),
            };

            let content_yml = serde_yaml::to_string(&config_yml)?;

            let mut new_config = fs::File::create(&paths.config_file_path)?;
            write!(new_config, "{}", content_yml)?;

            Ok(config_yml)
        }
    }

    pub fn get_redirect_uri(&self) -> String {
        format!("http://localhost:{}/callback", self.get_port())
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn get_user_agent(&self) -> String {
        match &self.user_agent {
            Some(s) => s.clone(),
            None => DEFAULT_USER_AGENT.to_string(),
        }
    }

    pub fn get_paths() -> Result<ConfigPaths, ConfigError> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(CONFIG_FILE);
                let token_cache_path = &app_config_dir.join(TOKEN_CACHE_FILE);

                let paths = ConfigPaths {
                    config_file_path: config_file_path.to_path_buf(),
                    auth_cache_path: token_cache_path.to_path_buf(),
                };

                Ok(paths)
            }
            None => Err(ConfigError::PathError),
        }
    }
}
