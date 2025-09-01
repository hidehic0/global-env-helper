use std::{env, fs, process::exit};

use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Parser)]
#[clap(

    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    arg_required_else_help = true,
    about ="A CLI tool for batch managing global environment variables in the shell."
)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    #[clap()]
    ShellHook {
        #[arg(long)]
        shell: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    envs: Vec<EnvConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
struct EnvConfig {
    key: String,
    value: String,
}

fn get_home_dir() -> Result<String, String> {
    return match env::var("HOME") {
        Ok(v) => Ok(v),
        Err(_) => Err("Failed to get home directory".to_string()),
    };
}

fn get_xdg_config_home() -> Result<String, String> {
    return match env::var("XDG_CONFIG_HOME") {
        Ok(v) => Ok(v),
        Err(_) => match get_home_dir() {
            Ok(v) => Ok(Path::new(&v)
                .join(".config")
                .to_str()
                .unwrap_or_else(|| "/")
                .to_string()),
            Err(v) => Err(v.to_string()),
        },
    };
}

fn get_config_path() -> Result<String, String> {
    return match get_xdg_config_home() {
        Ok(v) => match Path::new(&v)
            .join("global-env-helper")
            .join("config.toml")
            .to_str()
        {
            Some(v) => Ok(v.to_string()),
            None => Err("Failed to get config path".to_string()),
        },
        Err(v) => Err(v.to_string()),
    };
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        Subcommands::ShellHook { shell } => {
            let config_path = get_config_path().unwrap_or_else(|err| {
                println!("Error: {}", err);
                exit(1);
            });

            if !Path::new(&config_path).exists() {
                println!("Error: Config file not found");
                exit(1);
            }

            let _content: String = fs::read_to_string(config_path).unwrap();
            let config: Config = toml::from_str(&_content).unwrap();

            for env in config.envs {
                if shell == "zsh" || shell == "bash" {
                    println!("export {}={}", env.key, env.value)
                } else if shell == "fish" {
                    println!("set -x {} {}", env.key, env.value)
                }
            }
        }
    }
}
