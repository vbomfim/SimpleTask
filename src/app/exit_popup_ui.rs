use ratatui::text::Text;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use super::App;

// Editing
impl App {
    pub fn render_exit_popup(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(Clear, area, buf);
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = self.centered_rect(60, 25, area);
        exit_paragraph.render(area, buf);
    }
}
