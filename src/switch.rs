// Copyright (C) 2023 Félix Vadcard
// see main.rs for details

use config::Config;
use config::JavaVersion;
use std::fs;
use std::path::Path;

use std::env;
use std::process::Command;

use crate::config;
use crate::RunningPrompt;
use crate::MEMORY;

pub struct JavaVersionSwitcher {
    running_prompt: RunningPrompt,
    version: String,
    local: bool,
}

impl JavaVersionSwitcher {
    pub fn new(version: String, local: bool, running_prompt: RunningPrompt) -> JavaVersionSwitcher {
        JavaVersionSwitcher {
            running_prompt,
            version,
            local,
        }
    }
}

pub fn switch_java_version(java_version_switcher: JavaVersionSwitcher) {
    let mut memory = MEMORY.lock().expect("memory to be accessible");
    let config = &memory.config;
    let version = &java_version_switcher.version;
    let local = java_version_switcher.local;
    let running_prompt = java_version_switcher.running_prompt;

    let java_version = config
        .java_versions
        .get(&java_version_switcher.version)
        .expect("Chosen java version is not configured");

    if local {
        local_switch(&config, &version, &java_version, &running_prompt);
        println!("Java version was set to {} locally", version);
    } else {
        global_switch(&version, &java_version, &config, &running_prompt);
        memory.java_memory.current_version = version.to_string();
        memory.save();
        println!("Java version was set to {} globally", version);
    }
}

#[cfg(target_family = "windows")]
fn local_switch(
    config: &Config,
    version: &String,
    java_version: &JavaVersion,
    running_prompt: &RunningPrompt,
) {
    match running_prompt {
        RunningPrompt::Cmd => {
            let mut path = env::var("PATH").expect("Any environment should have a path");
            append_to_path(config, version, &mut path);
            write_temp_file_if_needed(java_version, path);
        }
        RunningPrompt::GitBash(_) => {
            change_symlink_git_bash(java_version);
        }
        RunningPrompt::Powershell => panic!("Powershell not supported yet"),
    }
}

#[cfg(target_family = "windows")]
fn change_symlink_git_bash(java_version: &JavaVersion) {
    let current_java_home = env::var("BFJVM_CURRENT_JAVA_HOME")
        .expect("BFJVM_CURRENT_JAVA_HOME env variable should always be set when using git_bash");
    fs::remove_dir_all(&current_java_home).expect("To be able to remove symlink directory");
    Command::new("ln")
        .arg("-sfn")
        .arg(&java_version.home_folder)
        .arg(&current_java_home)
        .spawn()
        .expect("To be able to create symlink to new java version");
}

fn write_temp_file_if_needed(java_version: &JavaVersion, path: String) {
    if let Some(tmp_file_path) = env::var("temp_file").ok() {
        let out = java_version.home_folder.to_string() + "|" + &path;
        let tmp_file_path = Path::new(&tmp_file_path);

        fs::write(
            &tmp_file_path
                .to_str()
                .expect("Temp file should be resolved on disk"),
            out,
        )
        .expect("Unable to write file");
    }
}

#[cfg(target_family = "windows")]
fn global_switch(
    version: &String,
    java_version: &JavaVersion,
    config: &Config,
    running_prompt: &RunningPrompt,
) {
    use std::collections::HashSet;

    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // create_subkey opens with write permissions
    let (env, _) = hkcu
        .create_subkey("environment")
        .expect("To be able to get evenvironement subkey as mutable");

    env.set_value("JAVA_HOME", &java_version.home_folder)
        .expect("JAVA_HOME should be mutable");
    let mut path: String = env
        .get_value("PATH")
        .expect("PATH should exist in any environment");
    setup_path(&version, &mut path, config);

    env.set_value("PATH", &path)
        .expect("PATH should be mutable");

    let path = match env::var("PATH").ok() {
        Some(envpath) => {
            let home = home_to_bin_os_path(&java_version.home_folder);
            let vec_java_home: Vec<&str> = vec![&home];
            let mut vec_paths: Vec<&str> = vec_java_home
                .into_iter()
                .chain(envpath.split(";"))
                .chain(path.split(";"))
                .collect();
            let mut seen = HashSet::new();
            vec_paths.retain(|item| seen.insert(*item));
            vec_paths.join(";")
        }
        None => path,
    };

    if let RunningPrompt::GitBash(_) = running_prompt {
        change_symlink_git_bash(java_version);
    }
    write_temp_file_if_needed(java_version, path);
}

#[cfg(target_family = "windows")]
fn setup_path(version: &String, path: &mut String, config: &Config) {
    //first we clean all remaining bin folder for all known java installations
    for java_version in config.java_versions.values() {
        *path = path.replace(&home_to_bin_os_path(&java_version.home_folder), "");
    }
    // then we append to pah and return
    append_to_path(config, version, path);
}

#[cfg(target_family = "windows")]
fn append_to_path(config: &Config, version: &String, path: &mut String) {
    let java_home_folder = config.get_javaversion(version).home_folder.to_string();
    *path = home_to_bin_os_path(&java_home_folder) + path;
}

#[cfg(target_family = "windows")]
fn home_to_bin_os_path(home_path: &String) -> String {
    let path = Path::new(home_path);
    let bin_path = path
        .join("bin")
        .to_str()
        .expect("Path to be return as str slice")
        .to_string();
    bin_path.replace("/", "\\") + ";"
}

#[cfg(target_family = "windows")]
fn home_to_cyg_path(home_path: &String) -> String {
    use crate::string_utils;

    let path = Path::new(home_path);
    let bin_path = path
        .join("bin")
        .to_str()
        .expect("Path to be return as str slice")
        .to_string();
    string_utils::win_to_cyg_path(&(bin_path + ";"))
}

#[cfg(test)]
mod tests {
    use crate::switch::home_to_bin_os_path;
    use std::assert_eq;

    #[test]
    fn home_to_bin_os_path_work() {
        let path = "C:/folder/folder with space/folder".to_string();
        let result = home_to_bin_os_path(&path);
        assert_eq!("C:\\folder\\folder with space\\folder\\bin;", result);
    }
}
