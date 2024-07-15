use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::Focused;

use super::StyledBorder;

#[derive(Copy, Clone)]
pub struct Weather {
    pub focus: Focused,
}

impl Widget for Weather {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.focus {
            Focused::Weather => self.focused(),
            _ => self.default(),
        }
        .title("Weather");

        block.render(area, buf);
    }
}
