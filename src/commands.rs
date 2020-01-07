use std::process::Command;

pub fn list_own_commits() {
    let result = Command::new("git").arg("log").arg("-s").output();

    match result {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(_) => panic!("could not execute command"),
    }
}
