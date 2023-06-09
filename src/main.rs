#[allow(dead_code, unused_imports)]
mod config;
mod switch;

use std::error::Error;

use clap::Parser;
use clap::Subcommand;
use config::load_config;
use switch::switch_java_version;

use crate::switch::JavaVersionSwitcher;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
    Init,
    /// Get informations from configuration
    Get {
        #[command(subcommand)]
        get: GetCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum GetCommands {
    /// Print the current config
    Config,
    /// Print your config path
    ConfigPath,
    /// Print informations about the java versions you are requesting
    Versions { version: String },
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

        #[arg(long, hide = true)]
        shell: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let config = load_config().expect("Config should load properly : ");

    match args.command {
        Commands::Java { java } => match java {
            JavaCommands::Switch {
                version,
                local,
                shell,
            } => switch_java_version(JavaVersionSwitcher::new(version, local, &config, shell)),
        },
        Commands::Init => (),
        Commands::Get { get } => {
            config.get(&get);
        }
    };
    Ok(())
}
