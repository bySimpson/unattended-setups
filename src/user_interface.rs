use std::io;

use crossbeam_channel::{unbounded, Receiver, Sender};
use crossterm::event::{self, Event, KeyCode};
use std::thread;
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::setup_manager::SetupManager;

pub enum EventListenerState {
    Listening,
    Sleeping,
}

pub struct UserInterface {
    pub titles: Vec<String>,
    pub index: usize,
    manager: SetupManager,
    pub show_popup: bool,
    pub popup_text: String,
}

impl UserInterface {
    pub fn new() -> Self {
        let mut manager = SetupManager::new(String::from("https://unattended-setups.thmr.at"));
        manager.update_setups();
        Self {
            titles: vec![String::from("install")],
            index: 0,
            manager,
            show_popup: false,
            popup_text: String::from(""),
        }
    }

    pub fn execute_current_setup(&mut self) {
        let setups = self.manager.get_setups();
        let current_script = setups
            .scripts
            .get(self.manager.state.selected().unwrap())
            .unwrap();
        let _ = current_script.execute();
        self.popup_text = format!("Successfully installed {}!", current_script.name);
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut interface: UserInterface,
) -> io::Result<()> {
    let (event_sender, event_receiver) = unbounded::<Event>();
    let (event_listener_sender, event_listener_receiver) = unbounded::<EventListenerState>();
    start_eventloop(event_sender, event_listener_receiver);
    loop {
        terminal.draw(|f| ui(f, &mut interface))?;

        for event in event_receiver.try_iter() {
            if !interface.show_popup {
                match event {
                    Event::Key(code) => match code.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => interface.manager.next_item(),
                        KeyCode::Up => interface.manager.previous_item(),
                        KeyCode::Char('u') => interface.manager.update_setups(),
                        //KeyCode::Char('p') => interface.show_popup = !interface.show_popup,
                        KeyCode::Enter => {
                            event_listener_sender.send(EventListenerState::Sleeping).unwrap();
                            interface.execute_current_setup();
                            event_listener_sender.send(EventListenerState::Listening).unwrap();
                            interface.show_popup = true;
                        }
                        _ => {}
                    },
                    _ => (),
                }
            } else {
                interface.show_popup = false;
            }
        }
    }
}

fn start_eventloop(
    event_sender: Sender<Event>,
    event_listener_receiver: Receiver<EventListenerState>,
) {
    let mut current_state = EventListenerState::Listening;
    let _ = thread::spawn(move || loop {
        let evt = event::read().unwrap();
        for c_lstnr_evt in event_listener_receiver.try_iter() {
            current_state = c_lstnr_evt;
        }
        match current_state {
            EventListenerState::Listening => event_sender.send(evt).unwrap(),
            EventListenerState::Sleeping => {}
        }
    });
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
                .add_modifier(Modifier::UNDERLINED)
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
        ).highlight_symbol(">> ");
    f.render_stateful_widget(setups_list, chunks[1], &mut interface.manager.state);
    if interface.show_popup {
        let block = Block::default().title("Status").borders(Borders::ALL);
        let paragraph = Paragraph::new(interface.popup_text.clone()).block(block);
        let area = centered_rect(60, 20, size);

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
