use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::Focused;

use super::StyledBorder;

// Build chart from below (variable station)
// https://environment.data.gov.uk/flood-monitoring/id/stations/E72639/readings?&today&_limit=100

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
