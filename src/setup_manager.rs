use crate::setup::{Setups};
use reqwest::Error;
use reqwest::blocking::{Client};
use reqwest::header::{USER_AGENT};

pub struct SetupManager {
    client: Client,
    api_url: String,
    setups: Option<Setups>
}

impl SetupManager {
    pub fn new(git_url: String) -> SetupManager {
        let client = Client::new();
        SetupManager {
            client: client,
            api_url: git_url,
            setups: None
        }
    }

    fn update_setups(&mut self) {
        match self.get_latest_setups() {
            Ok(setups) => {
                self.setups = Some(setups);
            },
            _ => ()
        };
    }

    fn get_latest_setups(&mut self) -> Result<Setups, Error> {
        let req = self
            .client
            .get(format!("{}/setup/all", &self.api_url))
            .header(USER_AGENT, "unattended-setups")
            .send()?
            .json::<Setups>()?;
        Ok(req)
    }

    pub fn get_setups(&mut self) -> Setups {
        self.update_setups();
        match &self.setups {
            Some(setups) => setups.clone(),
            None => Setups::new(vec![]),
        }
    }
}
