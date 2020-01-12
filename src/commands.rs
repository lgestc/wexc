use std::process::Command;

fn get_own_name() -> String {
    let output = Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .expect("could not get git user name");

    let name = String::from(String::from_utf8_lossy(&output.stdout).trim());

    return name;
}

#[derive(Clone, Debug)]
pub struct Commit {
    hash: String,
    subject: String,
}

fn get_author_commits<'a>(author: String) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            &["--author=", &author].join(""),
            "--format=\"%h||%s\"",
        ])
        .output()
        .expect("could not execute command");

    let output = String::from(String::from_utf8_lossy(&output.stdout));

    let lines = output.split("\n").collect::<Vec<&str>>();

    let mut commits: Vec<Commit> = Vec::new();

    for line in lines {
        let hash_and_subject = line.split("||").collect::<Vec<&str>>();
        let hash = match hash_and_subject.get(0) {
            Some(hash) => hash,
            None => "",
        };
        let hash = hash.trim();

        let subject = match hash_and_subject.get(1) {
            Some(subject) => subject,
            None => "",
        };
        let subject = subject.trim();

        if subject.chars().count() == 0 {
            continue;
        }

        if hash.chars().count() == 0 {
            continue;
        }

        commits.push(Commit {
            hash: String::from(hash),
            subject: String::from(subject),
        });
    }
    commits
}

pub fn list_own_commits() -> Vec<Commit> {
    get_author_commits(get_own_name())
}
