use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    config::{self, Config, CONFIG_FILE},
    proj_dirs::DATA_DIR,
    string_utils, GetCommands, GetCurrentCommands, RunningPrompt,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    pub static ref MEMORY_FILE: PathBuf = Path::join(&DATA_DIR, "bfjvm-memory");
    pub static ref ENV_PATH: String = env::var("PATH").expect("PATH to be set");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    config_hash: String,
    pub config: Config,
    pub java_memory: JavaMemory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaMemory {
    pub current_version: String,
}

impl Memory {
    fn default() -> Self {
        Memory {
            config_hash: "".to_string(),
            config: Config::default(),
            java_memory: JavaMemory {
                current_version: "".to_string(),
            },
        }
    }

    fn from(config: Config) -> Self {
        Memory {
            config_hash: "".to_string(),
            config,
            java_memory: JavaMemory {
                current_version: "".to_string(),
            },
        }
    }

    fn mergeconfig(self: &mut Self, config: Config) {
        self.config = config;
    }

    pub fn get(self: &Self, get: GetCommands, running_prompt: RunningPrompt) {
        match get {
            GetCommands::Current { current } => self.get_current(current, running_prompt),
            GetCommands::Config | GetCommands::ConfigPath => self.config.get(&get, running_prompt),
            GetCommands::Versions { version } => todo!(),
        };
    }

    fn get_current(self: &Self, get_cur: GetCurrentCommands, running_prompt: RunningPrompt) {
        match get_cur {
            GetCurrentCommands::JavaHome => {
                let java_version = self
                    .config
                    .java_versions
                    .get(&self.java_memory.current_version)
                    .expect("No current java version");

                let home_folder = match running_prompt {
                    RunningPrompt::GitBash(_) => {
                        string_utils::win_to_cyg_path(&java_version.home_folder)
                    }
                    _ => java_version.home_folder.to_string(),
                };
                println!("{}", home_folder);
            }
        }
    }

    pub fn save(self: &Self) {
        match dump_binaries(self) {
            Ok(()) => (),
            Err(err) => panic!("Could not save memory : {err}"),
        };
    }
}

pub fn initialize_memory() -> Memory {
    let mut memory: Memory;
    if !MEMORY_FILE.is_file() {
        let conf = match config::load_config() {
            Ok(conf) => conf,
            Err(err) => panic!("Cannot load config ! {err}"),
        };
        memory = init_memory_file(conf);
    } else {
        memory = load_memory_file();
        match read_hash_and_compare(&memory.config_hash) {
            Ok(are_equals) => {
                if !are_equals {
                    memory.mergeconfig(config::load_config().unwrap());
                    memory.save();
                }
            }
            Err(err) => panic!("Error could not compare config hash : {err}"),
        }
    }
    memory
}

fn init_memory_file(config: Config) -> Memory {
    let memory = Memory::from(config);
    match dump_binaries(&memory) {
        Ok(_) => memory,
        Err(err) => panic!("Cannot save memory ! {err}"),
    }
}

fn dump_binaries(memory: &Memory) -> Result<()> {
    fs::write(MEMORY_FILE.to_path_buf(), bincode::serialize(memory)?)?;
    Ok(())
}

fn load_memory_file() -> Memory {
    match load_from_binaries() {
        Ok(memory) => memory,
        Err(err) => panic!("Cannot load memory ! {err}"),
    }
}

fn load_from_binaries() -> Result<Memory> {
    let file = fs::read(MEMORY_FILE.to_path_buf())?;
    Ok(bincode::deserialize(&file)?)
}

fn read_hash_and_compare(saved_hash: &str) -> Result<bool> {
    let sha = sha256::try_digest(CONFIG_FILE.as_path())?;
    Ok(sha.as_str() == saved_hash)
}
