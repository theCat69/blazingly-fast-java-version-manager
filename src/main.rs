// blazingly fast java version manager a simple and efficinet java version manager
//
// Copyright (C) 2023 Félix Vadcard
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// You can contact the author regarding license issues using
// electronic mail at vadcard.felix@gmail.com

mod config;
mod init;
mod memory;
mod proj_dirs;
mod string_utils;
mod switch;
mod utility;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;

use clap::command;
use clap::Parser;
use clap::Subcommand;
use config::TMP_DIR;
use lazy_static::lazy_static;
use switch::switch_java_version;
use utility::print_rand_uuid;
use utility::print_win_to_cyg_path;

use crate::memory::initialize_memory;
use crate::memory::Memory;
use crate::switch::JavaVersionSwitcher;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    pub static ref MEMORY: Mutex<Memory> = Mutex::new(initialize_memory());
}

// We can choose different behavior in function of the os
// this is compile time choice so remember it
// We could do the same thing for clap parameters
// using "flatten" struct
#[cfg(target_family = "windows")]
pub static BIN_NAME: &str = "bf-j-vm[.bat|.sh]";

#[cfg(target_family = "linux")]
pub static BIN_NAME: &str = "bf-j-vm.sh";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[clap(bin_name = BIN_NAME)]
struct Cli {
    /// This is used internally to indicate which shell is
    /// running the application (hidden)
    #[arg(long, hide = true)]
    shell: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Java commands. Alias : "j"
    #[command(alias = "j")]
    Java {
        #[command(subcommand)]
        java: JavaCommands,
    },

    /// Redo initialisation of bf-j-vm
    #[command()]
    Init {
        #[command(subcommand)]
        init: InitCommands,
    },
    /// Get informations from configuration. Alias : "g"
    #[command(alias = "g")]
    Get {
        #[command(subcommand)]
        get: GetCommands,
    },

    /// Utility (hidden)
    #[command(hide = true)]
    Utility {
        #[command(subcommand)]
        utility: UtilityCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum InitCommands {
    /// Initialize git_bash you might be need to run it after updating bfjvm version
    GitBash,
    /// That clean and redo init of bfjvm. This will make backup of your configuration files (TODO)
    Full,
}

#[derive(Debug, Subcommand)]
pub enum UtilityCommands {
    /// Get a random UUID identifier
    RandUuid,
    /// Print to cygpath
    #[command(arg_required_else_help = true)]
    WinToCygPath { path: String },
    /// For testing purposes can be anything
    Test,
}

#[derive(Debug, Subcommand)]
pub enum GetCommands {
    /// Print the current config
    Config,
    /// Print your config path
    ConfigPath,
    /// Print informations about the java versions you are requesting
    Versions { version: String },
    /// Get the current configured version
    Current {
        #[command(subcommand)]
        current: GetCurrentCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum GetCurrentCommands {
    /// Get current java_home system path
    #[command(alias = "jh")]
    JavaHome,
}

#[derive(Debug, Subcommand)]
pub enum JavaCommands {
    /// Switch your java versions. Alias : "s"
    #[command(arg_required_else_help = true, alias = "s")]
    Switch {
        /// The version to swicth to
        version: String,
        /// Add this switch to change the version locally only
        #[arg(short, long)]
        local: bool,
    },
}

#[cfg(target_family = "windows")]
pub enum RunningPrompt {
    Cmd,
    GitBash(PathBuf),
    Powershell,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let running_prompt = match args.shell {
        Some(shell_str) => match shell_str.as_str() {
            "powershell" => RunningPrompt::Powershell,
            "git_bash" => RunningPrompt::GitBash(TMP_DIR.to_path_buf()),
            _ => RunningPrompt::Cmd,
        },
        None => RunningPrompt::Cmd,
    };

    match args.command {
        Commands::Java { java } => match java {
            JavaCommands::Switch { version, local } => {
                switch_java_version(JavaVersionSwitcher::new(version, local, running_prompt))
            }
        },
        Commands::Init { init } => match init {
            InitCommands::GitBash => init::init_git_bash(running_prompt),
            InitCommands::Full => init::init(running_prompt).unwrap(),
        },
        Commands::Get { get } => {
            MEMORY
                .lock()
                .expect("memory to be accessible")
                .get(get, running_prompt);
        }
        Commands::Utility { utility } => match utility {
            UtilityCommands::RandUuid => print_rand_uuid(),
            UtilityCommands::Test => (),
            UtilityCommands::WinToCygPath { path } => print_win_to_cyg_path(path.as_str()),
        },
    };
    Ok(())
}
