use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result, 
    cursor::{MoveTo, Show, Hide}
};
use std::{
    error::Error,
    io::{stdout, Write},
    time::{Duration, Instant},
};
mod chessboard;
use crate::chessboard::Board;

// don't change this, it works well.
fn main() -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Hide,
        EnterAlternateScreen,
    )?;
    enable_raw_mode()?;

    let board = Board::new(vec![ // TODO: turn this into a macro
        'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R', 
        'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P', 
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 
        'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', 
        'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'
    ]);

    let ticks = Duration::from_millis(10);

    loop {
        execute!(stdout(), MoveTo(0, 0))?;
        match read()? {
            Event::Key(KeyEvent{
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE, ..
            }) => break,
            _ => (),
        }
    }

    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        Show,
        ResetColor
    )?;
    return Ok(());
}
