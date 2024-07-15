use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

mod app;
pub mod panes;
use app::{App, Focused, RunningState, SearchMode, View};
use panes::{search::Search, Pane, PaneLayout, SearchLayout};

#[derive(PartialEq, Eq)]
enum Input {
    Add(char),
    Remove,
}

#[derive(PartialEq, Eq)]
enum Message {
    SearchInput(Input),
    Cycle,
    Escape,
    Transition,
    Quit,
    Other,
}

fn main() -> io::Result<()> {
    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;
    let mut app = App::default();
    let mut search = Search::new();
    let mut pane = Pane::build(&mut app, &mut search);

    while app.state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut pane, &mut search, &mut app, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&app)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut pane, &mut search, &mut app, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(pane: &mut Pane, search: &mut Search, app: &mut App, f: &mut Frame) {
    let pane_layout = PaneLayout::build(f);
    let search_layout = SearchLayout::build(f);
    let focus = app.focus;

    pane.sunset.focus = focus;
    pane.weather.focus = focus;
    pane.tide.focus = focus;

    match app.view {
        View::Search(SearchMode::Editing) => {
            search.mode = SearchMode::Editing;
            f.render_widget(search, search_layout.area);
        }

        View::Search(SearchMode::Normal) => {
            search.mode = SearchMode::Normal;
            f.render_widget(search, search_layout.area)
        }

        View::Enlarged => {
            match focus {
                Focused::Sunset => f.render_widget(pane.sunset, pane_layout.full),
                Focused::Tide => f.render_widget(&mut pane.tide, pane_layout.full),
                Focused::Weather => f.render_widget(pane.weather, pane_layout.full),
            };
        }

        View::Compressed => {
            pane.tide.get_station_readings();

            f.render_widget(&mut pane.tide, pane_layout.bottom);
            f.render_widget(pane.sunset, pane_layout.top_right);
            f.render_widget(pane.weather, pane_layout.top_left);
        }
    }
}

fn handle_event(app: &App) -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(app, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(app: &App, key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Tab => return Some(Message::Cycle),
        KeyCode::Enter => return Some(Message::Transition),
        _ => {}
    }

    let msg = match app.view {
        View::Search(SearchMode::Editing) => match key.code {
            KeyCode::Char(ch) => Message::SearchInput(Input::Add(ch)),
            KeyCode::Backspace => Message::SearchInput(Input::Remove),
            KeyCode::Esc => Message::Escape,
            _ => Message::Other,
        },

        View::Search(SearchMode::Normal) => match key.code {
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Other,
        },

        _ => match key.code {
            KeyCode::Char('q') => Message::Quit,
            KeyCode::Esc => Message::Escape,
            _ => Message::Other,
        },
    };

    if msg == Message::Other {
        None
    } else {
        Some(msg)
    }
}

fn update(pane: &mut Pane, search: &mut Search, app: &mut App, msg: Message) -> Option<Message> {
    match app.view {
        View::Search(SearchMode::Editing) => match msg {
            Message::Cycle | Message::Escape => app.toggle_search(&search.mode),
            Message::Transition => {
                search.transition(app);

                let station_reference = search
                    .station
                    .as_ref()
                    .and_then(|f| Some(f.station_reference.clone()));

                pane.tide.station_reference = station_reference;
            }
            Message::SearchInput(Input::Add(ch)) => search.add_char(ch),
            Message::SearchInput(Input::Remove) => search.remove_char(),
            _ => {}
        },

        View::Search(SearchMode::Normal) => match msg {
            Message::Cycle => app.toggle_search(&search.mode),
            Message::Transition => search.transition(app),
            Message::Quit => app.quit(),
            _ => {}
        },

        _ => match msg {
            Message::Cycle => app.cycle(),
            Message::Quit => app.quit(),
            Message::Transition => app.transition(),
            Message::Escape => app.escape(),
            _ => {}
        },
    }

    None
}

mod tui {
    use crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use ratatui::prelude::*;
    use std::io;
    use std::{io::stdout, panic};

    pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> io::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
