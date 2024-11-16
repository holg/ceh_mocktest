// src/helper/feature_manager.rs
use std::process::Command;
use clap::{Arg, Command as ClapCommand};
use colored::*;
use anyhow;
#[derive(Debug)]
pub struct AppConfig {
    pub use_local: bool,
    pub use_ki: bool,
    pub use_clipboard: bool,
    pub use_py: bool,
    pub use_db: bool,
    pub use_sqlite: bool,
}

impl AppConfig {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        AppConfig {
            use_local: matches.get_flag("use_local"),
            use_ki: matches.get_flag("use_ki"),
            use_clipboard: matches.get_flag("use_clipboard"),
            use_py: matches.get_flag("use_py"),
            use_db: matches.get_flag("use_db"),
            use_sqlite: matches.get_flag("use_sqlite"),
        }
    }
}

pub fn get_app_config() -> AppConfig {
    let matches = build_clap_app().get_matches();
    AppConfig::new(&matches)
}

pub fn get_features_description(config: &AppConfig) -> anyhow::Result<Vec<ColoredString>> {
    let mut features = vec![];

    if config.use_local {
        features.push("use_local".green());
    }
    if config.use_ki {
        features.push("use_ki".cyan());
    }
    if config.use_clipboard {
        features.push("use_clipboard".red());
    }
    if config.use_py {
        features.push("use_py".italic());
    }
    if config.use_db {
        features.push("use_db".bright_green());
    }
    if config.use_sqlite {
        features.push("use_sqlite".yellow());
    }

    Ok(features)
}

pub fn banner(config: &AppConfig) -> anyhow::Result<()> {
    let art = r#"
    _____ ______ _    _      __  __            _      _______        _
  / ____|  ____| |  | |    |  \/  |          | |    |__   __|      | |
 | |    | |__  | |__| |    | \  / | ___   ___| | __    | | ___  ___| |_
 | |    |  __| |  __  |    | |\/| |/ _ \ / __| |/ /    | |/ _ \/ __| __|
 | |____| |____| |  | |    | |  | | (_) | (__|   <     | |  __/\__ \ |_
  \_____|______|_|  |_|    |_|  |_|\___/ \___|_|\_\    |_|\___||___/\__|
                                                                   v1.5
    Quiz by @TmS'htr_'hsuCryptographic

 For Certified Ethical Hacker Trainees            Last Updated November 2024
----------------------------------------------------------------------------
    "#;
    println!("{}", art);

    let features_description = get_features_description(&config)?;
    let string_descriptions: Vec<String> = features_description.iter().map(ToString::to_string).collect();
    println!("Available features: {}", string_descriptions.join(" -f"));
    Ok(())
}

pub fn build_clap_app() -> clap::Command {
    ClapCommand::new("FeatureManager")
        .version("0.5")
        .author("Holger Trahe <holg@github.com>")
        .about("Manages features for the application")
        .arg(
            Arg::new("use_local")
                .long("use_local")
                .help("Enables use_local feature")
        )
        .arg(
            Arg::new("use_ki")
                .long("use_ki")
                .help("Enables use_ki feature")
        )
        .arg(
            Arg::new("use_clipboard")
                .long("use_clipboard")
                .help("Enables use_clipboard feature")
        )
        .arg(
            Arg::new("use_py")
                .long("use_py")
                .help("Enables use_py feature")
        )
        .arg(
            Arg::new("use_db")
                .long("use_db")
                .help("Enables use_db feature")
        )
        .arg(
            Arg::new("use_sqlite")
                .long("use_sqlite")
                .help("Enables use_sqlite feature")
        )
}
