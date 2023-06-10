use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{config::Config, proj_dirs::DATA_DIR};

lazy_static! {
    pub static ref MEMORY_FILE: PathBuf = Path::join(&DATA_DIR, "bfjvm-memory");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    config: Config,
    java_memory: JavaMemory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaMemory {
    current_version: String,
}

// pub fn initialize_memory() {
//     let memory: Memory;
//     if !MEMORY_FILE.is_file() {
//         let conf = match config::load_config() {
//             Ok(conf) => conf,
//             Err(err) => panic!("Cannot load config ! {err}"),
//         };
//         memory = init_memory_file(config)?;
//     } else {
//         config = load_from_memory()?;
//     }
// }
//
// fn init_memory_file() -> _ {
//     todo!()
// }
