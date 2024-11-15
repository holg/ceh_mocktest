//helpers/mod.rs
use std::process::Command;

pub mod htr_low_level_http;
pub mod ollama;
pub mod quiz;
mod ceh_lab;

#[cfg(not(feature = "use_local"))]
pub const USE_LOCAL: bool = false;
#[cfg(feature = "use_local")]
pub const USE_LOCAL: bool = true;
#[cfg(not(feature = "use_ki"))]
pub const USE_KI: bool = false;
#[cfg(feature = "use_ki")]
pub const USE_KI: bool = true;


#[allow(dead_code)]
pub fn apple_say_using(text: &str, voice_using: Option<&str>) {
    let script = format!(r#"osascript -e 'say "{}" using "{})"'"#, text, voice_using.unwrap_or("Tingting"));
    dbg!(&script);
    if let Err(e) = Command::new("sh").arg("-c").arg(&script).status() {
        eprintln!("Failed to execute osascript: {}", e);
    }
}

pub fn banner() {
    let art = r#"
    _____ ______ _    _      __  __            _      _______        _
  / ____|  ____| |  | |    |  \/  |          | |    |__   __|      | |
 | |    | |__  | |__| |    | \  / | ___   ___| | __    | | ___  ___| |_
 | |    |  __| |  __  |    | |\/| |/ _ \ / __| |/ /    | |/ _ \/ __| __|
 | |____| |____| |  | |    | |  | | (_) | (__|   <     | |  __/\__ \ |_
  \_____|______|_|  |_|    |_|  |_|\___/ \___|_|\_\    |_|\___||___/\__|
                                                                   v1.5
    Quiz by @TS'htr_'hsuCryptographic

 For Certified Ethical Hacker v12                    Last Updated April 2024
----------------------------------------------------------------------------
    "#;
    println!("{}", art);
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

/// Returns true if the "use_py" feature is enabled, otherwise false.
pub fn debug_default_level() -> u32 {
    10
}

