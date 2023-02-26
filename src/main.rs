// vim:foldmethod=marker:ft=rust
// IMPORTS {{{
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result, 
    cursor::{MoveTo, Show, Hide}
};
use std::{
    io::{stdout, Write},
};
// mod chessboard;
// use crate::chessboard::Board;
mod board;
use crate::board::Board;
pub mod pieces {
    pub mod chess_piece;
    pub mod king;
    pub mod pawn;
    pub mod knight;
    pub mod queen;
    pub mod bishop;
    pub mod rook;
}
use crate::pieces::chess_piece::chess_piece::Piece;
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

    let mut board = Board::new(&[
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'], 
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'], 
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r']
    ]);

    print_board_grid(
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

    board.request_move(&(1,1), &(1,3));

    show_pieces(
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

    draw_input_box(2,1)?;


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

// draw_input_box {{{
fn draw_input_box(x: u16, y: u16) -> Result<()> {
    execute!(stdout(),
        MoveTo(x,y),
        Print("╭───────╮"),
        MoveTo(x,y+1),
        Print("│       │"),
        MoveTo(x,y+2),
        Print("╰───────╯")
    )?;
    stdout().flush()?;
    Ok(())
}
// }}}
// show_pieces {{{
fn show_pieces(x: u16, y: u16, num_cols: u8, num_rows: u8, column_width: u8, row_height: u8, board: &Board, color_a: Color, color_b: Color) -> Result<()> {
    for row in 0u8..num_rows as u8 {
        for col in 0..num_cols {
            // let piece = display_version_of.get(&board.state[row*num_cols+col]).unwrap_or(&board.state[row*num_cols+col]);
            let piece = if let Some(&c)=board.get_at(&(col, row)) {c.display_char()} else {' '};
            execute!(stdout(),
                MoveTo(x+(col*column_width) as u16, y+(row*row_height) as u16),
                SetBackgroundColor(if (col+row)%2==0 {color_a} else {color_b}),
                SetForegroundColor(if (col+row)%2==0 {color_b} else {color_a}),
                Print(piece),
                ResetColor
            )?;
        }
    }

    stdout().flush()?;
    Ok(())
}
// }}}
// board printing {{{
fn print_board_grid(x: u16, 
                  y: u16, 
                  numrows: usize, 
                  numcols: usize, 
                  cellwidth: usize, 
                  cellheight: usize,
                  file_labels: String, 
                  rank_labels: String,
                  color_a: Color,
                  color_b: Color, 
                  border_color: Color
                  ) -> Result<()> {

    //display the top border
    execute!(stdout(), 
        MoveTo(x,y), 
        SetBackgroundColor(border_color), 
        Print(format!("{: <1$}\n\r", "", 4+cellwidth*numcols)),
        ResetColor,
        Print(format!("{: <1$}", "", x as usize))
    )?;

    //loop through every rank and draw each row of the board
    for rank in 0..(numrows*cellheight) {
        execute!(stdout(),
            SetBackgroundColor(border_color),
            Print(format!("{} ", if rank%cellheight==cellheight/2 {file_labels.chars().nth(rank/cellheight).unwrap()} else {' '})),
        )?;
        for file in 0..numcols {
            execute!(stdout(),
                SetBackgroundColor(
                    if (rank/cellheight + file)%2==0 {color_a} 
                    else {color_b}
                ),
                Print(format!("{: <1$}", "", cellwidth)),
            )?;
        }
        execute!(stdout(), 
            SetBackgroundColor(border_color),
            Print("  "),
            ResetColor,
            Print(format!("\n\r{: <1$}", "", x as usize))
        )?;
    }

    //draw the final border with the legends
    execute!(stdout(), 
        SetBackgroundColor(border_color), 
        Print("  ")
    )?;
    for column in 0..numcols {
        execute!(stdout(),
            Print(format!("{: <1$}", "", cellwidth/2)),
            Print(format!("{}", rank_labels.chars().nth(column).unwrap())),
            Print(format!("{: <1$}", "", cellwidth/2-1))
        )?;
    }
    execute!(stdout(), Print("  "), ResetColor)?;
    
    stdout().flush()?;
    Ok(())
}
// }}}
