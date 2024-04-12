mod speech;
mod misc;

use speech::*;
use misc::*;

fn main() {
    setup_check();
    println!("INFO: Starting play...");
    play("Hello! This is a test phrase. I am curious how long I can make an input string? Rust is an interesting programming language.");
    println!("INFO: Play done.");
}
