use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result
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
        EnterAlternateScreen,
    )?;

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
        if crossterm::event::poll(ticks)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    execute!(
                        stdout(),
                        LeaveAlternateScreen,
                        ResetColor
                    )?;
                    return Ok(());
                }
            }
        }
    }

}
