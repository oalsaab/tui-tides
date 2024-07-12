use ratatui::{
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders},
};

pub mod search;
pub mod sunset;
pub mod tide;
pub mod weather;

use search::Search;
use sunset::Sunset;
use tide::Tide;
use weather::Weather;

trait StyledBorder {
    fn focused(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Cyan)
            .title_style(Color::White)
    }

    fn default(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::DarkGray)
            .title_style(Color::White)
    }
}

impl StyledBorder for Search {}
impl StyledBorder for Weather {}
impl StyledBorder for Sunset {}
impl StyledBorder for Tide {}
