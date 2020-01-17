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

    fn generate_interactive_selection_file(&self, entries: &Vec<Entry>) -> String {
        let mut temp_output = String::new();

        temp_output.push_str("# Replace \"default\" to alter excerpt report file name. Otherwise, it will be constructed using first picked entry.\n");
        temp_output.push_str("Output filename: default\n\n");

        temp_output.push_str("# Prefix given entry with \"p \" to include it in the report \n\n");

        entries.iter().for_each(|entry| {
            temp_output.push_str(&entry.id.to_owned());
            temp_output.push_str(": ");
            temp_output.push_str(&entry.subject.to_owned());
            temp_output.push_str(" (");
            temp_output.push_str(&entry.timestamp.to_owned());
            temp_output.push_str(")\n");
        });

        temp_output
    }

    fn parse_selection<'a>(
        &self,
        entries: &'a Vec<Entry>,
        interactive_selection_file: &String,
    ) -> Vec<&'a Entry> {
        let picked_ids: Vec<String> = interactive_selection_file
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

        let matching_entries: Vec<&Entry> = entries
            .iter()
            .filter(|entry| picked_ids.contains(&entry.id))
            .collect();

        matching_entries
    }

    fn prepare_report_path(
        &self,
        selected_entries: &Vec<&Entry>,
        interactive_selection_file: &String,
    ) -> String {
        let report_file_name = interactive_selection_file
            .lines()
            .filter(|line| line.contains("Output filename:"))
            .last()
            .and_then(|line| Some(String::from(line)))
            .unwrap();

        let report_file_name: Vec<&str> = report_file_name.split(":").collect();

        let report_file_name = report_file_name.get(1).unwrap_or(&"default");

        let report_file_name = String::from(report_file_name.trim());

        let report_file_name = if report_file_name == "default" {
            let first_entry = selected_entries
                .first()
                .expect("no matching entries available to build default report name");

            return String::from(&first_entry.subject);
        } else {
            report_file_name
        };

        let mut report_file_path = current_dir().unwrap();
        report_file_path.push(report_file_name);

        let report_file_path = String::from(report_file_path.as_path().to_str().unwrap());

        report_file_path
    }
}

impl Renderer for Cli {
    fn render(&self, provider: impl Provider) {
        let entries = provider.provide_entries();

        let editor = var("EDITOR").unwrap();
        let wait_arg = if editor.contains("code") {
            String::from("--wait")
        } else {
            String::new()
        };
        let mut interactive_selection_file_path = temp_dir();
        interactive_selection_file_path.push("pick entries");

        let mut interactive_selection_file = self.generate_interactive_selection_file(&entries);

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&interactive_selection_file_path)
            .expect("could not open file")
            .write_all(interactive_selection_file.as_bytes())
            .expect("could not write interactive selection temp file");

        Command::new(editor)
            .arg(&wait_arg)
            .arg(&interactive_selection_file_path)
            .spawn()
            .expect("could not open interactive selection editor")
            .wait_with_output()
            .expect("could not gather editor output");

        File::open(&interactive_selection_file_path)
            .expect("could not reopen interactive selection file")
            .read_to_string(&mut interactive_selection_file)
            .expect("could not read modified interactive selection file");

        let selected_entries = self.parse_selection(&entries, &interactive_selection_file);

        if selected_entries.len() == 0 {
            println!("no entries selected, aborting");
            return;
        }

        let report = provider.report_for_entries(&selected_entries);

        let report_file_path =
            self.prepare_report_path(&selected_entries, &interactive_selection_file);

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(report_file_path)
            .expect("could not open report file")
            .write_all(report.as_bytes())
            .expect("could not write report file");
    }
}
