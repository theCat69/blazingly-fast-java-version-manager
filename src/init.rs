// Copyright (C) 2023 Félix Vadcard
// see main.rs for details

pub mod directory_autocomplete;

use std::fs::OpenOptions;
use std::io::{BufWriter, Read};
use std::process::Command;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::init::directory_autocomplete::InitAutocomplete;
use crate::proj_dirs::CONFIG_DIR;
use crate::string_utils;
use crate::{Result, RunningPrompt};

use inquire::Text;
use lazy_static::lazy_static;
use walkdir::{DirEntry, WalkDir};

lazy_static! {
    pub static ref GIT_BASH_INIT_SCRIPT_DIR: PathBuf = Path::join(&CONFIG_DIR, "git_bash_init");
}

#[derive(Debug, Default)]
pub struct JavaInstallation {
    java_home: String,
    version: String,
    runtime: String,
    short_description: String,
    java_version_out: String,
    distribution: JavaDistribution,
}

#[derive(Debug)]
pub enum JavaDistribution {
    GraalVM,
    HotSpot,
    Temurin,
    RedHat,
    OpenLogic,
    Microsoft,
    Zulu,
    OpenJdk,
}

impl JavaDistribution {
    pub fn as_str(self: &Self) -> &str {
        match self {
            JavaDistribution::GraalVM => "GraalVM",
            JavaDistribution::HotSpot => "HotSpot",
            JavaDistribution::Temurin => "Temurin",
            JavaDistribution::RedHat => "RedHat",
            JavaDistribution::OpenLogic => "OpenLogic",
            JavaDistribution::Microsoft => "Microsoft",
            JavaDistribution::Zulu => "Zulu",
            JavaDistribution::OpenJdk => "OpenJdk",
        }
    }

    pub fn from<S: Into<String>>(str: S) -> Self {
        match str.into() {
            graal_vm if graal_vm.contains("GraalVM") => JavaDistribution::GraalVM,
            temurin if temurin.contains("Temurin") => JavaDistribution::Temurin,
            zulu if zulu.contains("Zulu") => JavaDistribution::Zulu,
            open_logic if open_logic.contains("OpenLogic") => JavaDistribution::OpenLogic,
            microsoft if microsoft.contains("Microsoft") => JavaDistribution::Microsoft,
            hotspot if hotspot.contains("HotSpot") => JavaDistribution::HotSpot,
            red_hat if red_hat.contains("Red_Hat") => JavaDistribution::RedHat,
            _ => JavaDistribution::default(),
        }
    }
}

impl Default for JavaDistribution {
    fn default() -> Self {
        JavaDistribution::OpenJdk
    }
}

pub fn init(running_prompt: RunningPrompt) -> Result<()> {
    println!("Starting initalisation of bf-j-vm");
    println!("Searching for java installations");
    let folder_input = Text::new("Please type the folder you want me to search into :")
        .with_autocomplete(InitAutocomplete::default())
        .prompt()?;
    let vec_dir_en = search_for_java_installation(folder_input.as_str());
    let java_installs = build_java_installations(vec_dir_en)?;
    // println!("{:?}", &java_installs);
    for j_i in java_installs {
        println!("");
        println!("{:?}", j_i.version);
        println!("{:?}", j_i.distribution);
        println!("{:?}", j_i.java_home);
        println!("{:?}", j_i.runtime);
        println!("{:?}", j_i.short_description);
        println!("{:?}", j_i.java_version_out);
    }
    Ok(())
}

fn build_java_installations(vec_dir_en: Vec<DirEntry>) -> Result<Vec<JavaInstallation>> {
    let mut java_installs: Vec<JavaInstallation> = Vec::new();
    for entry in vec_dir_en {
        if let Some(out) = Command::new(entry.path()).arg("-version").output().ok() {
            let java_version_out: String = String::from_utf8(out.stderr)?;
            if let Some(java_install) = extract_java_installation(java_version_out, entry) {
                java_installs.push(java_install);
            }
        }
    }
    Ok(java_installs)
}

fn extract_java_installation(
    java_version_out: String,
    entry: DirEntry,
) -> Option<JavaInstallation> {
    let mut version_out_lines = java_version_out.split("\n");

    let version = version_out_lines.next()?.split("\"").nth(1)?.to_string();

    let short_description = version_out_lines.next()?.to_string();

    let parent_path = entry.path().parent()?;

    let runtime = match Command::new(Path::join(parent_path, "javac.exe"))
        .arg("-h")
        .output()
    {
        Ok(_) => "jdk".to_string(),
        Err(_) => "jre".to_string(),
    };

    let distribution = JavaDistribution::from(&java_version_out);

    let java_home = match parent_path.file_name()?.to_str()? {
        "bin" => parent_path.parent()?.to_str()?.to_string(),
        _ => parent_path.to_str()?.to_string(),
    };

    Some(JavaInstallation {
        java_home,
        version,
        runtime,
        short_description,
        java_version_out,
        distribution,
    })
}

fn search_for_java_installation(base_dir: &str) -> Vec<DirEntry> {
    let mut vec_to_keep: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(base_dir.trim())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry
            .path()
            .to_str()
            .is_some_and(|s| s.ends_with("java.exe"))
        {
            vec_to_keep.push(entry);
        }
    }
    vec_to_keep
}

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
