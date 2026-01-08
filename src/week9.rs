use std::process::Command;

pub fn tasks() {
    println!("hi");
    let output = Command::new("cmd").args(["/C", "dir"]).output().expect("Failed to execute command");

    let mut child = Command::new("sleep").arg("5").spawn().expect("failed to spawn");

    let status = child.wait().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
