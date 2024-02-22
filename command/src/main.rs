#![allow(unused)]
fn main() {
use std::process::Command;

let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .expect("failed to execute process")
} else {
    Command::new("ls")
        .arg("-la")
        // .arg("echo hello")
        .output()
        .expect("failed to execute process")
};

let hello = String::from_utf8_lossy(&output.stdout);
if output.status.success() {
    println!("{}", hello);
} else {
    let error_str = String::from_utf8_lossy(&output.stderr);
    eprintln!("Error: {}", error_str);
}
}
