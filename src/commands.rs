use std::process::Command;

fn get_own_name() -> String {
    let result = Command::new("git").args(&["config", "user.name"]).output();

    let name = match result {
        Ok(output) => String::from(String::from_utf8_lossy(&output.stdout).trim()),
        Err(e) => {
            eprintln!("{}", e);
            panic!("could not get git user.name")
        }
    };

    return name;
}

#[derive(Clone, Debug)]
pub struct Commit {
    hash: String,
    subject: String,
}

fn get_author_commits<'a>(author: String) -> Vec<Commit> {
    let result = Command::new("git")
        .args(&[
            "log",
            &["--author=", &author].join(""),
            "--format=\"%h||%s\"",
        ])
        .output();

    let output = match result {
        Ok(output) => String::from(String::from_utf8_lossy(&output.stdout)),
        Err(e) => {
            eprintln!("{}", e);
            panic!("could not execute command")
        }
    };

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
