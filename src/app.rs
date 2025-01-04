use std::collections::HashMap;

use ratatui::prelude::*;

use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{widgets::Widget, Frame};

use crate::TerminalStdErr;
mod editing_ui;
mod exit_popup_ui;
mod main_ui;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
pub enum CurrentlyEditing {
    Key,
    Value,
}
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub exit: bool,
}
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut TerminalStdErr) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        serde_json::Result::Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                KeyCode::Char('e') => {
                    self.current_screen = CurrentScreen::Editing;
                    self.currently_editing = Some(CurrentlyEditing::Key);
                }
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                _ => {}
            },
            CurrentScreen::Editing => match key_event.code {
                KeyCode::Enter => {
                    if let Some(editing) = &self.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                self.currently_editing = Some(CurrentlyEditing::Value);
                            }
                            CurrentlyEditing::Value => {
                                self.save_key_value();
                                self.current_screen = CurrentScreen::Main;
                            }
                        }
                    }
                }
                KeyCode::Tab => {
                    self.toggle_editing();
                }
                KeyCode::Backspace => {
                    if let Some(editing) = &self.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                self.key_input.pop();
                            }
                            CurrentlyEditing::Value => {
                                self.value_input.pop();
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    self.current_screen = CurrentScreen::Main;
                    self.currently_editing = None;
                }
                KeyCode::Char(value) => {
                    if let Some(editing) = &self.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                self.key_input.push(value);
                            }
                            CurrentlyEditing::Value => {
                                self.value_input.push(value);
                            }
                        }
                    }
                }
                _ => {}
            },
            CurrentScreen::Exiting => match key_event.code {
                KeyCode::Char('q') => {
                    // match self.print_json() {
                    //     serde_json::Result::Err(err) => {
                    //         print!("{}", err)
                    //     }
                    //     _ => "",
                    // }
                    return self.exit();
                }
                _ => {} // KeyCode::Char('n') | KeyCode::Char('q') => {
                        //     return Ok(false);
                        // }
            },
        }
        // match key_event.code {
        //     KeyCode::Char('q') => self.exit(),
        //     _ => {}
        // }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_main(area, buf);
        if let CurrentScreen::Editing = &self.current_screen {
            self.render_editing(area, buf);
        }
        // Exit Popup
        if let CurrentScreen::Exiting = self.current_screen {
            self.render_exit_popup(area, buf);
        }
    }
}
