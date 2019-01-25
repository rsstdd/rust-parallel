// -- ##################################
// -- Task: To call a child process
// -- ##################################

// Use std thread crate
use std::process::Command;

// Main thread starts
fn main() {
    // Command to be executed
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });

    // printing the output value
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        println!("rustc succeeded and stdout was:n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        println!("rustc failed and stderr was:n{}", s);
    }
}
