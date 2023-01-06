use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
mod logo;
pub struct Selections {
    pub toolselection: [String],
}

impl Selections {
    pub fn new() {
        let toolselections = &["React", "React Native", "Node", "Flutter"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select build: ")
            .default(0)
            .items(&toolselections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            if toolselections[selection] == "React Native" {
                let mut list_dir = Command::new("yarn");
                list_dir.spawn().expect("FAILED");
                list_dir.current_dir("../../insulet-mobile");
            } else {
                panic!("WRONG ARG");
            }
        } else {
            panic!("Did not select something");
        }
    }
}
