use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
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
    enable_raw_mode()?; // allows us to get the keypresses without the user having to press "enter"
                        // like in a regular terminal

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

    printBoardGrid(5, 5)?;

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

fn printBoardGrid(x: u16, y: u16) -> Result<()> {
    let NUMROWS = 8;
    let NUMCOLS = 8;
    let CELLWIDTH = 6;
    let CELLHEIGHT = 3;

    execute!(stdout(), MoveTo(x,y))?;
    for rank in 0..(NUMROWS*CELLHEIGHT) {
        for _ in 0..(NUMCOLS/2) { // <- THIS will produce a bug where odd number of cols dont work.
            execute!(stdout(),
                SetBackgroundColor(if (rank/CELLHEIGHT)%2==0 {Color::White} else {Color::Black}),
                Print(format!("{: <1$}", "", CELLWIDTH)),
                SetBackgroundColor(if (rank/CELLHEIGHT)%2==0 {Color::Black} else {Color::White}),
                Print(format!("{: <1$}", "", CELLWIDTH)),
                ResetColor
            )?;
        }
        print!("\n\r{: <1$}", "", x as usize);
    }
    
    stdout().flush();
    Ok(())
}
