use super::renderer::Renderer;
use crate::backend::provider::Provider;
use crate::model::entry::Entry;

use std::{
    env::{current_dir, temp_dir, var},
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
        let mut entries = provider.provide_entries();
        let mut temp_output = String::new();

        entries.reverse();

        temp_output.push_str("# Replace \"work_excerpt\" to alter excerpt report file name:\n");
        temp_output.push_str("Output filename: work_excerpt\n\n");

        temp_output.push_str("# Prepend given entry with \"p \" to include it in the report \n\n");

        entries.iter().for_each(|entry| {
            temp_output.push_str(&entry.id.to_owned());
            temp_output.push_str(": ");
            temp_output.push_str(&entry.subject.to_owned());
            temp_output.push_str("\n");
        });

        let editor = var("EDITOR").unwrap();
        let mut file_path = temp_dir();
        file_path.push("pick entries");
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
            .filter(|line| !line.contains("#"))
            .filter(|line| !line.contains("Output filename:"))
            .filter(|line| line.find("p ").unwrap_or(1) == 0)
            .map(|line| {
                let hash = line.replacen("p ", "", 1);
                let hash = hash.split(":").next().unwrap();
                return String::from(hash);
            })
            .collect();

        if picked_ids.len() == 0 {
            println!("no items selected, aborting");
            return;
        }

        let matching_entries: Vec<&Entry> = entries
            .iter()
            .filter(|entry| picked_ids.contains(&entry.id))
            .collect();

        let mut output = String::new();

        matching_entries.iter().for_each(|entry| {
            let mut parent_ref = String::from(&entry.id);
            parent_ref.push_str("^");

            let git_output = Command::new("git")
                .arg("diff")
                .arg(parent_ref)
                .arg(&entry.id)
                .output()
                .expect("could not execute command");

            let git_output = String::from(String::from_utf8_lossy(&git_output.stdout));

            output.push_str(&git_output);
        });

        let report_name_line = updated_file
            .lines()
            .filter(|line| line.contains("Output filename:"))
            .last()
            .and_then(|line| Some(String::from(line)))
            .unwrap();

        let report_name_line: Vec<&str> = report_name_line.split(":").collect();

        let report_name_line = report_name_line
            .get(1)
            .expect("Could not get report name. Please do not remove \"Output filename:\" prefix");

        let report_file_name = String::from(report_name_line.trim());

        let mut report_file_name = String::from(report_file_name);
        report_file_name.push_str(".diff");

        let mut report_file = current_dir().unwrap();
        report_file.push(report_file_name);

        let output_filename = String::from(report_file.as_path().to_str().unwrap());

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_filename)
            .expect("could not open file")
            .write_all(output.as_bytes())
            .expect("could not write");
    }
}
