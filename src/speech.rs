use super::*;

pub fn play(snippet: &str) {
    let path = std::path::Path::new("/home/exeo/eve/");
    std::process::Command::new("edge-tts")
        .current_dir(path)
        .args([
            "--text", snippet, 
            "--write-media", "temp.mp3"
        ])
        .output()
        .expect("ERROR: Failed to write audio data to 'temp.mp3'");
    std::process::Command::new("ffplay")
        .current_dir(path)
        .args([
            "-v", "0",
            "-nodisp",
            "-autoexit",
            "temp.mp3"
        ])
        .output()
        .expect("ERROR: Is ffmpeg installed?");
    std::process::Command::new("rm")
        .current_dir(path)
        .arg("temp.mp3")
        .output()
        .expect("ERROR: Failed to delete temp.mp3");

}
