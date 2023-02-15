use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setups {
    pub scripts: Vec<Setup>,
}

impl Setups {
    pub fn new(scripts: Vec<Setup>) -> Self {
        Self { scripts }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setup {
    pub name: String,
    pub path: String,
}

impl Setup {
    pub fn execute(&self) -> String {
        let output = Command::new("sh")
        .arg("-c")
        .arg(self.get_text())
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn get_text(&self) -> String {
        Client::new().get(format!("{}", &self.path)).send().expect("Could not download script!").text().expect("Did not receive message!")
    }
}
