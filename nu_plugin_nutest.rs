use std::process::Command;

fn main() {
    let nushell_command = r#"clear"#;

    let output = Command::new("nu")
        .arg("-c")
        .arg(nushell_command)
        .output();

    match output {
        Ok(output_data) => {
            if output_data.status.success() {
                let stdout = String::from_utf8_lossy(&output_data.stdout);
                let stderr = String::from_utf8_lossy(&output_data.stderr);
                println!("Command executed successfully. Output:");
                println!("Stdout:\n{}", stdout);
                println!("Stderr:\n{}", stderr);
            } else {
                println!("Command failed to execute.");
            }
        }
        Err(e) => {
            println!("Error executing command: {}", e);
        }
    }
}

