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
use crate::{config::Config, RunningPrompt};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_BASH_INIT_SCRIPT_DIR: PathBuf = Path::join(&CONFIG_DIR, "git_bash_init");
}

pub fn init(config: &Config, running_prompt: RunningPrompt) {}

pub fn init_git_bash() {
    let init_dir = GIT_BASH_INIT_SCRIPT_DIR.to_path_buf();
    fs::create_dir_all(&init_dir).expect("To be able to create git bash init dir");
    let init_file_name = "bfjvm-init-git-bash.sh";
    let init_file_path = Path::join(&init_dir, init_file_name);
    let mut file = File::create(&init_file_path).expect("To be able to create init file");

    let script_path = env::var("BFJVM_SCRIPTPATH").expect("BFJVM_SCRIPTPATH to be set");
    let mut buff: Vec<u8> = Vec::new();
    File::open(Path::new(&script_path).join("scripts").join(init_file_name))
        .expect("To be able to read from original init git bash file")
        .read_to_end(&mut buff)
        .expect("To be able to read content and write to buffer");
    file.write(&buff).expect("To be able to wite to init file");

    let home = env::var("HOME").expect("HOME env to be set");

    let bashrc = home + "/.bashrc";
    let bashrc_content = fs::read_to_string(&bashrc).expect("To be able to read bashrc content");
    //TODO delete this part from the file
    match bashrc_content.contains("##### BFJVM FLAG #####") {
        true => replace_bfjvm_flag(bashrc, bashrc_content, init_file_path),
        false => write_to_end_of_file(bashrc, init_file_path),
    }

    println!("Git bash was intilialized please restart your prompt")
}

fn replace_bfjvm_flag(bashrc: String, bashrc_content: String, init_file_path: PathBuf) {
    let (bashrc_user_content, _) = bashrc_content
        .split_once("##### BFJVM FLAG #####")
        .expect("Should be able to split at that point");
    let finalize_content =
        bashrc_user_content.to_string() + &get_bfjvm_bashrc_text(&init_file_path);
    fs::write(bashrc, finalize_content).expect("To be able to write back to bashrc");
}

fn write_to_end_of_file(bashrc: String, init_file_path: PathBuf) {
    let file = OpenOptions::new().append(true).open(bashrc);
    match file {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            let _ = writer.write_all((get_bfjvm_bashrc_text(&init_file_path)).as_bytes());
        }
        Err(err) => panic!("Error occured will initializing git bash quitting ... {err}"),
    };
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
