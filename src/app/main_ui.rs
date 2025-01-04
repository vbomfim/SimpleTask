use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Widget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use super::{App, CurrentScreen, CurrentlyEditing};

impl App {
    pub fn render_main(&self, area: Rect, buf: &mut Buffer) {
        // Split the area in three horizontal chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        // Main window design
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        Paragraph::new(Text::styled(
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block)
        .render(chunks[0], buf);

        // Populates the KVP List in the Main Window
        let mut list_items = Vec::<ListItem>::new();
        for (key, value) in &self.pairs {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25} : {}", key, value),
                Style::default().fg(Color::Yellow),
            ))));
        }
        let list = List::new(list_items);
        Widget::render(list, chunks[1], buf);

        let current_navigation_text = vec![
            // The first half of the text
            match self.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(Color::Green))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
                }
                CurrentScreen::Exiting => {
                    Span::styled("Exiting", Style::default().fg(Color::LightRed))
                }
            }
            .to_owned(),
            // A white divider bar to separate the two sections
            Span::styled(" | ", Style::default().fg(Color::White)),
            // The final section of the text, with hints on what the user is editing
            {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];

        // Footer
        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));
        let current_keys_hint = {
            match self.current_screen {
                CurrentScreen::Main => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Editing => Span::styled(
                    "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
            }
        };

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        mode_footer.render(footer_chunks[0], buf);
        key_notes_footer.render(footer_chunks[1], buf);
    }
}
