use std::fs::OpenOptions;
use std::io::{BufWriter, Read};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::proj_dirs::CONFIG_DIR;
use crate::string_utils;
use crate::{config::Config, Result, RunningPrompt};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_BASH_INIT_SCRIPT_DIR: PathBuf = Path::join(&CONFIG_DIR, "git_bash_init");
}

pub fn init(config: &Config, running_prompt: RunningPrompt) {}

pub fn init_git_bash(running_prompt: RunningPrompt) {
    match running_prompt {
        RunningPrompt::GitBash(_) => (),
        _ => panic!("This method should only be called in a git bash prompt"),
    }

    let init_file_path = match create_git_bash_init_file() {
        Ok(file_path) => file_path,
        Err(err) => panic!("Cannot setup bfjvm git bash init file : {err}"),
    };

    let home = env::var("HOME").expect("HOME env to be set");

    let bashrc = home + "/.bashrc";
    let bashrc_content = fs::read_to_string(&bashrc).expect("To be able to read bashrc content");
    //TODO delete this part from the file
    let bashrc_result = match bashrc_content.contains("##### BFJVM FLAG #####") {
        true => replace_bfjvm_flag(bashrc, bashrc_content, init_file_path),
        false => write_to_end_of_file(bashrc, init_file_path),
    };

    match bashrc_result {
        Ok(()) => println!("Git bash was intilialized please restart your prompt"),
        Err(err) => panic!("Error occured during git bash init : {err}"),
    }
}

fn create_git_bash_init_file() -> Result<PathBuf> {
    let init_dir = GIT_BASH_INIT_SCRIPT_DIR.to_path_buf();
    fs::create_dir_all(&init_dir)?;
    let init_file_name = "bfjvm-init-git-bash.sh";
    let init_file_path = Path::join(&init_dir, init_file_name);
    let mut file = File::create(&init_file_path)?;

    let script_path = env::var("BFJVM_SCRIPTPATH")?;
    let mut buff: Vec<u8> = Vec::new();
    File::open(Path::new(&script_path).join("scripts").join(init_file_name))?
        .read_to_end(&mut buff)?;
    file.write(&buff)?;
    Ok(init_file_path)
}

fn replace_bfjvm_flag(
    bashrc: String,
    bashrc_content: String,
    init_file_path: PathBuf,
) -> Result<()> {
    let (bashrc_user_content, _) = bashrc_content
        .split_once("##### BFJVM FLAG #####")
        .expect("Should be able to split at that point");
    let finalize_content =
        bashrc_user_content.to_string() + &get_bfjvm_bashrc_text(&init_file_path);
    fs::write(bashrc, finalize_content)?;
    Ok(())
}

fn write_to_end_of_file(bashrc: String, init_file_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().append(true).open(bashrc)?;
    let mut writer = BufWriter::new(file);
    writer.write_all((get_bfjvm_bashrc_text(&init_file_path)).as_bytes())?;
    Ok(())
}

fn get_bfjvm_bashrc_text(init_file_path: &PathBuf) -> String {
    "##### BFJVM FLAG #####

export BFJVM_INIT_FILE=\""
        .to_string()
        + &string_utils::win_to_cyg_path(init_file_path.to_str().unwrap())
        + "\"
[[ -z \"${BFJVM_INIT_FILE}\" ]] || source \"${BFJVM_INIT_FILE}\"

##### END BFJVM FLAG #####"
}
