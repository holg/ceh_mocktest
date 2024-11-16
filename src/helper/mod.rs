//helpers/mod.rs
use std::process::Command;
use colored::*;
pub mod htr_low_level_http;
pub mod ollama;
pub mod quiz;
mod ceh_lab;
mod json_to_sqlite;
mod feature_manager;
pub use feature_manager::{AppConfig, build_clap_app, get_app_config};

#[cfg(feature = "use_sqlite")]
pub use json_to_sqlite::json_to_sqlite;
use crate::helper;

#[cfg(not(feature = "use_local"))]
pub const USE_LOCAL: bool = false;
#[cfg(feature = "use_local")]
pub const USE_LOCAL: bool = true;
#[cfg(not(feature = "use_ki"))]
pub const USE_KI: bool = false;
#[cfg(feature = "use_ki")]
pub const USE_KI: bool = true;

pub fn get_features_description() -> anyhow::Result<Vec<ColoredString>> {
    let features = vec![
        "use_local".green(),
        "do_quiz".blue(),
        "use_ki".cyan(),
        "use_clipboard".red(),
        "use_py".italic(),
        "use_db".bright_green(),
        "use_sqlite".yellow()
    ];
    Ok(features)

}


#[allow(dead_code)]
pub fn apple_say_using(text: &str, voice_using: Option<&str>) {
    let script = format!(r#"osascript -e 'say "{}" using "{})"'"#, text, voice_using.unwrap_or("Tingting"));
    dbg!(&script);
    if let Err(e) = Command::new("sh").arg("-c").arg(&script).status() {
        eprintln!("Failed to execute osascript: {}", e);
    }
}

/// Returns true if the "use_local" feature is enabled, otherwise false.
pub fn is_use_local() -> bool {
    cfg!(feature = "use_local")
}

/// Returns true if the "use_ki" feature is enabled, otherwise false.
pub fn is_use_ki() -> bool {
    cfg!(feature = "use_ki")
}
/// Returns true if the "use_clipboard" feature is enabled, otherwise false.
pub fn is_use_clipboard() -> bool {
    cfg!(feature = "use_clipboard")
}

/// Returns true if the "use_py" feature is enabled, otherwise false.
pub fn is_use_py() -> bool {
    cfg!(feature = "use_py")
}

/// Returns true if the "use_db" feature is enabled, otherwise false.
pub fn is_use_db() -> bool {
    cfg!(feature = "use_db")
}

/// Returns true if the "use_sqlite" feature is enabled, otherwise false.
pub fn is_use_sqlite() -> bool {
    cfg!(feature = "use_sqlite")
}

/// Returns true if the "use_py" feature is enabled, otherwise false.
pub fn debug_default_level() -> i8 {
    10
}

