use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use super::{App, CurrentlyEditing};

// Editing
impl App {
    pub fn render_editing(&self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = self.centered_rect(60, 25, area);
        popup_block.render(area, buf);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        if let Some(currently_editing) = &self.currently_editing {
            match currently_editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_style),
            };
        }
        let key_text = Paragraph::new(self.key_input.clone()).block(key_block);
        key_text.render(popup_chunks[0], buf);

        let value_text = Paragraph::new(self.value_input.clone()).block(value_block);
        value_text.render(popup_chunks[1], buf);
    }
}
