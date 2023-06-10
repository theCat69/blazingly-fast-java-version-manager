use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROJ_DIR: ProjectDirs = init_proj_dir();
    pub static ref RUNTIME_DIR: PathBuf = lazy_init_dirs(PROJ_DIR.runtime_dir());
    pub static ref DATA_DIR: PathBuf = lazy_init_dirs(Some(PROJ_DIR.data_dir()));
    pub static ref CONFIG_DIR: PathBuf = lazy_init_dirs(Some(PROJ_DIR.config_dir()));
}

fn init_proj_dir() -> ProjectDirs {
    match ProjectDirs::from("rs", "", "bf-j-vm") {
        Some(proj_dirs) => proj_dirs,
        None => panic!("Error creating config dir"),
    }
}

fn lazy_init_dirs(runtime_dir: Option<&std::path::Path>) -> PathBuf {
    match runtime_dir {
        Some(run_dir) => match fs::create_dir_all(run_dir) {
            Ok(()) => run_dir.to_path_buf(),
            Err(err) => panic!("Error creating config dir : {err}"),
        },
        //TODO what is fallback ?
        None => panic!("I should be able to create dir"),
    }
}
