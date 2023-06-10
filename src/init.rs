use std::io::{BufRead, BufReader, BufWriter, Read};
use std::os::windows::prelude::FileExt;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    config::{Config, CONFIG_DIR},
    RunningPrompt,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIT_BASH_INIT_SCRIPT_DIR: PathBuf = Path::join(&CONFIG_DIR, "git_bash_init");
}

pub fn init(config: &Config, running_prompt: RunningPrompt) {}

pub fn init_git_bash(config: &Config) {
    let init_dir = GIT_BASH_INIT_SCRIPT_DIR.to_path_buf();
    fs::create_dir_all(&init_dir).expect("To be able to create git bash init dir");
    let init_file_name = "bfjvm-init-git-bash.sh";
    let mut file = File::create(Path::join(&init_dir, init_file_name))
        .expect("To be able to create init file");

    let script_path = env::var("BFJVM_SCRIPTPATH").expect("BFJVM_SCRIPTPATH to be set");
    let mut buff: Vec<u8> = Vec::new();
    File::open(Path::new(&script_path).join("scripts").join(init_file_name))
        .expect("To be able to read from original init git bash file")
        .read(&mut buff)
        .expect("To be able to read content and write to buffer");
    file.write(&buff).expect("To be able to wite to init file");

    let home = env::var("HOME").expect("HOME env to be set");
    let file = File::open(home + "/.bashrc");
    match file {
        Ok(_f) => {
            //     let reader = BufReader::new(f);
            //     while let Some(line) = reader.read_line(buf).ok() {
            //           if let Some(l) = line.ok() {
            //             if l == "##### BFJVM FLAG #####" {
            //                 panic!("Git bash is already initializie !");
            //             }
            //         }
            //
            //     }
            //     for line in reader.li() {
            //         if let Some(l) = line.ok() {
            //             if l == "##### BFJVM FLAG #####" {
            //                 panic!("Git bash is already initializie !");
            //             }
            //         }
            //     }
            //     let writer = BufWriter::new(f);
            //     vec_to_write.push("##### BFJVM FLAG #####".to_string().as_bytes());
            //     vec_to_write.push("##### END BFJVM FLAG #####".to_string().as_bytes());
            //
            //     f.seek_write(buf, reader.)
        }
        //##### BFJVM FLAG #####
        //
        // export BFJVM_INIT_FILE="a path"
        // [[ -z "${BFJVM_INIT_FILE}" ]] && source "${BFJVM_INIT_FILE}"
        //
        // #### END BFJVM FLAG #####
        Err(err) => panic!("Error occured will initializing git bash quitting ... {err}"),
    };
}
