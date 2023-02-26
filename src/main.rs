// vim:foldmethod=marker:ft=rust
// IMPORTS {{{
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
    collections::HashMap
};
// mod chessboard;
// use crate::chessboard::Board;
mod board;
use crate::board::Board;
pub mod Pieces {
    pub mod chessPiece;
    pub mod king;
    pub mod pawn;
    pub mod knight;
    pub mod queen;
    pub mod bishop;
    pub mod rook;
}
use crate::Pieces::chessPiece::chessPiece::Piece;
// pub mod board {
//     use crate::Pieces::{
//         chessPiece::chessPiece::{Square, ChessPiece, Piece},
//         king::King::King,
//         pawn::Pawn::Pawn,
//         knight::Knight::Knight,
//         queen::Queen::Queen,
//         bishop::Bishop::Bishop,
//         rook::Rook::Rook
//     };
// }
// }}}

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

    let board = Board::new(&[
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'], 
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'], 
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r']
    ]);

    printBoardGrid(
        10, // x pos
        5,  // y pos
        8,  // num rows vertically
        8,  // num columns horizontally
        6,  // width of each cell
        3,  // height of each cell
        "ABCDEFGH".to_string(), 
        "12345678".to_string(), 
        Color::White, 
        Color::Black, 
        Color::Rgb{r:117, g:83, b:24})?;
    showPieces(
        15, // x pos
        7,  // y pos
        8,  // num cols
        8,  // num rows
        6,  // width of each cell
        3,  // height of each cell
        &board, // the board
        Color::White,
        Color::Black
    )?;

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

fn showPieces(x: u16, y: u16, numCols: u8, numRows: u8, colWidth: u8, rowHeight: u8, board: &Board, COLOR_A: Color, COLOR_B: Color) -> Result<()> {
    for row in 0u8..numRows as u8 {
        for col in 0..numCols {
            // let piece = display_version_of.get(&board.state[row*numCols+col]).unwrap_or(&board.state[row*numCols+col]);
            let piece = if let Some(&c)=board.get_at(&(col, row)) {c.displayChar()} else {' '};
            execute!(stdout(),
                MoveTo(x+(col*colWidth) as u16, y+(row*rowHeight) as u16),
                SetBackgroundColor(if (col+row)%2==0 {COLOR_A} else {COLOR_B}),
                SetForegroundColor(if (col+row)%2==0 {COLOR_B} else {COLOR_A}),
                Print(piece),
            )?;
        }
    }

    stdout().flush();
    Ok(())
}
// board printing {{{
fn printBoardGrid(x: u16, 
                  y: u16, 
                  numrows: usize, 
                  numcols: usize, 
                  cellwidth: usize, 
                  cellheight: usize,
                  fileLabels: String, 
                  rankLabels: String,
                  COLOR_A: Color,
                  COLOR_B: Color, 
                  BORDERCOLOR: Color
                  ) -> Result<()> {

    //display the top border
    execute!(stdout(), 
        MoveTo(x,y), 
        SetBackgroundColor(BORDERCOLOR), 
        Print(format!("{: <1$}\n\r", "", 4+cellwidth*numcols)),
        ResetColor,
        Print(format!("{: <1$}", "", x as usize))
    )?;

    //loop through every rank and draw each row of the board
    for rank in 0..(numrows*cellheight) {
        execute!(stdout(),
            SetBackgroundColor(BORDERCOLOR),
            Print(format!("{} ", if rank%cellheight==cellheight/2 {fileLabels.chars().nth(rank/cellheight).unwrap()} else {' '})),
        )?;
        for file in 0..numcols {
            execute!(stdout(),
                SetBackgroundColor(
                    if (rank/cellheight + file)%2==0 {COLOR_A} 
                    else {COLOR_B}
                ),
                Print(format!("{: <1$}", "", cellwidth)),
            )?;
        }
        execute!(stdout(), 
            SetBackgroundColor(BORDERCOLOR),
            Print("  "),
            ResetColor,
            Print(format!("\n\r{: <1$}", "", x as usize))
        )?;
    }

    //draw the final border with the legends
    execute!(stdout(), 
        SetBackgroundColor(BORDERCOLOR), 
        Print("  ")
    )?;
    for column in 0..numcols {
        execute!(stdout(),
            Print(format!("{: <1$}", "", cellwidth/2)),
            Print(format!("{}", rankLabels.chars().nth(column).unwrap())),
            Print(format!("{: <1$}", "", cellwidth/2-1))
        )?;
    }
    execute!(stdout(), Print("  "), ResetColor)?;
    
    stdout().flush()?;
    Ok(())
}
// }}}
