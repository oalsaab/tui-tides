use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::Focused;

use super::StyledBorder;

pub struct Sunset {
    pub focus: Focused,
}

impl Widget for Sunset {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.focus {
            Focused::Sunset => self.focused(),
            _ => self.default(),
        }
        .title("Sunset");

        block.render(area, buf);
    }
}
