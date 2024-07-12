use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::Focused;

use super::StyledBorder;

pub struct Tide {
    pub focus: Focused,
}

impl Widget for Tide {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.focus {
            Focused::Tide => self.focused(),
            _ => self.default(),
        }
        .title("Tide");

        block.render(area, buf);
    }
}
