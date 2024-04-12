use super::*;
use std::io::Write;

pub fn setup_check() {
    let path = std::path::Path::new("/home/exeo/eve/");
    if path.exists() {
        println!("INFO: |/home/exeo/eve | dir already existed...");
        println!("INFO: Maybe there was a previous install?");
    } else {
        println!("INFO: Creating dir...");
        std::fs::create_dir(path).expect("ERROR: Failed to create dir :(");
    }
    let output = std::process::Command::new("pip")
        .args([
            "show", "edge-tts"
        ])
        .output()
        .expect("ERROR: Failed to check whether or not edge-tts is installed...");
    if String::from_utf8_lossy(&output.stderr) != "" {
        println!("INFO: 'edge-tts' is not installed");
        println!("INFO: Attempting to install it...");
        std::process::Command::new("pip")
            .args([
                "install", "edge-tts"
            ])
            .output()
            .expect("ERROR: Failed to 'pip install edge-tts'");
    }
}
