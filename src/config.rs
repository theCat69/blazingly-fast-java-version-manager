use core::fmt;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;

use std::io::Write;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::write;
use std::{collections::HashMap, fs, println};

use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::GetCommands;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    pub static ref CONFIG_DIR: PathBuf = init_config_dir();
    pub static ref CONFIG_FILE: PathBuf = Path::join(&CONFIG_DIR, "bf-j-vm.json");
    pub static ref BFJVM_DIR: PathBuf = Path::join(&CONFIG_DIR, "current");
    pub static ref TMP_DIR: PathBuf = Path::join(&CONFIG_DIR, "tmp");
}

fn init_bfjvm_dir(base_dir: &PathBuf) -> PathBuf {
    let bfjvm_dir = Path::join(base_dir, "current");
    fs::create_dir_all(&bfjvm_dir).expect("To be able to create bfjvm directory");
    bfjvm_dir
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaVersion {
    pub home_folder: String,
    pub installed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_current_jdk: PathBuf,
    pub java_versions: HashMap<String, JavaVersion>,
}

impl Config {
    pub fn get(&self, command: &GetCommands) {
        match command {
            GetCommands::Config => println!(
                "{}",
                serde_json::to_string_pretty(self)
                    .expect("To be able to deserialize configuration")
            ),
            GetCommands::ConfigPath => println!("{}", CONFIG_DIR.to_string_lossy()),
            GetCommands::Versions { version } => println!("Printing version {}", version),
        };
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        let java_versions: HashMap<String, JavaVersion> = HashMap::new();
        Config {
            user_current_jdk: BFJVM_DIR.to_path_buf(),
            java_versions,
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config: Config;
    if !CONFIG_FILE.is_file() {
        config = init_config_file()?;
    } else {
        config = load_config_from_file()?
    }
    Ok(config)
}

fn load_config_from_file() -> Result<Config> {
    let file = fs::read(CONFIG_FILE.to_path_buf())?;
    Ok(serde_json::from_slice::<Config>(&file)?)
}

fn init_config_file() -> Result<Config> {
    let config = Config::default();
    let json_config = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(CONFIG_FILE.to_path_buf())?;
    file.write_all(json_config.as_bytes())?;
    Ok(config)
}

fn init_config_dir() -> PathBuf {
    match ProjectDirs::from("rs", "", "bf-j-vm") {
        Some(proj_dirs) => {
            let config_directory = proj_dirs.config_dir();
            match fs::create_dir_all(config_directory) {
                Ok(()) => config_directory.to_path_buf(),
                Err(err) => panic!("Error creating config dir : {err}"),
            }
        }
        None => panic!("Error creating config dir"),
    }
}
