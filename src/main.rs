use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

mod app;
pub mod panes;
use app::{App, Focused, RunningState, SearchMode, View};
use panes::{search::Search, sunset::Sunset, tide::Tide, weather::Weather};

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

    while app.state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut search, &mut app, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&app)?;

        // let app = App::new();

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut search, &mut app, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

struct PaneLayout {
    full: Rect,
    top_right: Rect,
    top_left: Rect,
    bottom: Rect,
}

impl PaneLayout {
    fn build(f: &mut Frame) -> PaneLayout {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[0]);

        PaneLayout {
            full: layout[0],
            top_right: top[1],
            top_left: top[0],
            bottom: main[1],
        }
    }
}

struct SearchLayout {
    area: Rect,
}

impl SearchLayout {
    fn build(f: &mut Frame) -> SearchLayout {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let search_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(layout[0]);

        // Centre rect into layout[0]
        // First figure out the text

        SearchLayout {
            area: search_box[0],
        }
    }
}

fn view(search: &mut Search, app: &mut App, f: &mut Frame) {
    let panes = PaneLayout::build(f);
    let search_layout = SearchLayout::build(f);
    let focus = app.focus;

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
                Focused::Sunset => f.render_widget(Sunset { focus }, panes.full),
                Focused::Tide => f.render_widget(Tide { focus }, panes.full),
                Focused::Weather => f.render_widget(Weather { focus }, panes.full),
            };
        }

        View::Compressed => {
            f.render_widget(Tide { focus }, panes.bottom);
            f.render_widget(Sunset { focus }, panes.top_right);
            f.render_widget(Weather { focus }, panes.top_left);
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

fn update(search: &mut Search, app: &mut App, msg: Message) -> Option<Message> {
    match app.view {
        View::Search(SearchMode::Editing) => match msg {
            // Change transition to perform execution, and accept app
            Message::Cycle | Message::Escape => app.toggle_search(&search.mode),
            Message::Transition => search.execute(),
            Message::SearchInput(Input::Add(ch)) => search.add_char(ch),
            Message::SearchInput(Input::Remove) => search.remove_char(),
            _ => {}
        },

        View::Search(SearchMode::Normal) => match msg {
            Message::Cycle => app.toggle_search(&search.mode),
            Message::Transition => search.execute(),
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
