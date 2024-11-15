//helpers/mod.rs
use std::process::Command;

pub mod htr_low_level_http;

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
