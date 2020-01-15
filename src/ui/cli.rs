use super::renderer::Renderer;
use crate::backend::provider::Provider;
use crate::model::entry::Entry;

use std::{
    env::{temp_dir, var},
    fs::File,
    fs::OpenOptions,
    io::Read,
    io::Write,
    process::Command,
};

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
}

impl Renderer for Cli {
    fn render(&self, provider: impl Provider) {
        let entries = provider.provide_entries();
        let mut temp_output = String::new();

        for entry in &entries {
            temp_output.push_str(&entry.id.to_owned());
            temp_output.push_str(": ");
            temp_output.push_str(&entry.subject.to_owned());
            temp_output.push_str("\n");
        }

        let editor = var("EDITOR").unwrap();
        let mut file_path = temp_dir();
        file_path.push("editable");
        File::create(&file_path).expect("Could not create file");

        let wait_arg = if editor.contains("code") {
            String::from("--wait")
        } else {
            String::new()
        };

        OpenOptions::new()
            .write(true)
            .open(&file_path)
            .expect("could not open file")
            .write_all(temp_output.as_bytes())
            .expect("could not write");

        Command::new(editor)
            .arg(&wait_arg)
            .arg(&file_path)
            .spawn()
            .expect("could not spawn command")
            .wait_with_output()
            .expect("could not execute command");

        let mut updated_file = String::new();

        File::open(&file_path)
            .expect("Could not open file")
            .read_to_string(&mut updated_file)
            .expect("could not read file");

        let picked_ids: Vec<String> = updated_file
            .lines()
            .filter(|line| line.find("p ").unwrap_or(1) == 0)
            .map(|line| {
                let hash = line.replacen("p ", "", 1);
                let hash = hash.split(":").next().unwrap();
                return String::from(hash);
            })
            .collect();

        let matching_entries: Vec<&Entry> = entries
            .iter()
            .filter(|entry| picked_ids.contains(&entry.id))
            .collect();

        print!("{:?}", matching_entries);
    }
}
