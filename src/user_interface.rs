use std::io;

use crossterm::event::{self, Event, KeyCode};
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Frame, Terminal,
};

use crate::setup_manager::SetupManager;

pub struct UserInterface {
    pub titles: Vec<String>,
    pub index: usize,
    manager: SetupManager,
}

impl UserInterface {
    pub fn new() -> Self {
        let mut manager = SetupManager::new(String::from("https://unattended-setups.thmr.at"));
        manager.update_setups();
        Self {
            titles: vec![String::from("install")],
            index: 0,
            manager,
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut interface: UserInterface,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut interface))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => interface.manager.next_item(),
                KeyCode::Up => interface.manager.previous_item(),
                KeyCode::Char('u') => interface.manager.update_setups(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, interface: &mut UserInterface) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let titles = interface
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(t, Style::default())))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Commands"))
        .select(interface.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        );
    f.render_widget(tabs, chunks[0]);

    let setups: Vec<ListItem> = interface
        .manager
        .get_setups()
        .scripts
        .iter()
        .map(|c_script| ListItem::new(c_script.name.to_owned()))
        .collect();
    let setups_list = List::new(setups)
        .block(Block::default().borders(Borders::ALL).title("Setups"))
        .style(Style::default())
        .start_corner(Corner::TopLeft)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        );
    f.render_stateful_widget(setups_list, chunks[1], &mut interface.manager.state);
}
