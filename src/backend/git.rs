use super::provider::Provider;

use crate::model::entry::Entry;

use std::process::Command;

pub struct GitProvider {}

impl GitProvider {
    pub fn new() -> GitProvider {
        GitProvider {}
    }

    fn get_own_name(&self) -> String {
        let output = Command::new("git")
            .args(&["config", "user.name"])
            .output()
            .expect("could not get git user name");
        let name = String::from(String::from_utf8_lossy(&output.stdout).trim());
        return name;
    }

    fn get_commits(&self, author: String) -> Vec<Entry> {
        let output = Command::new("git")
            .args(&[
                "log",
                &["--author=", &author].join(""),
                "--format=%h||%s||%ai",
            ])
            .output()
            .expect("could not execute command");
        let output = String::from(String::from_utf8_lossy(&output.stdout));
        let lines = output.split("\n").collect::<Vec<&str>>();
        let mut commits: Vec<Entry> = Vec::new();
        for line in lines {
            let entry_data = line.split("||").collect::<Vec<&str>>();
            let hash = match entry_data.get(0) {
                Some(hash) => hash,
                None => "",
            };
            let hash = hash.trim();
            let subject = match entry_data.get(1) {
                Some(subject) => subject,
                None => "",
            };
            let subject = subject.trim();

            let timestamp = match entry_data.get(2) {
                Some(timestamp) => timestamp,
                None => "",
            };
            let timestamp = timestamp.trim();

            if subject.chars().count() == 0 {
                continue;
            }
            if hash.chars().count() == 0 {
                continue;
            }
            commits.push(Entry {
                id: String::from(hash),
                subject: String::from(subject),
                timestamp: String::from(timestamp),
            });
        }
        commits
    }
}

impl Provider for GitProvider {
    fn provide_entries(&self) -> Vec<Entry> {
        self.get_commits(self.get_own_name())
    }
}
