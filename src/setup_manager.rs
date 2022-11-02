use crate::setup::Setups;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::Error;
use tui::widgets::ListState;

pub struct SetupManager {
    client: Client,
    api_url: String,
    setups: Option<Setups>,
    pub state: ListState,
}

impl SetupManager {
    pub fn new(api_url: String) -> SetupManager {
        let client = Client::new();
        SetupManager {
            client,
            api_url,
            setups: None,
            state: ListState::default(),
        }
    }

    pub fn update_setups(&mut self) {
        if let Ok(setups) = self.get_latest_setups() {
            self.setups = Some(setups);
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
        match &self.setups {
            Some(setups) => setups.clone(),
            None => Setups::new(vec![]),
        }
    }

    pub fn next_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.get_setups().scripts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.get_setups().scripts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
