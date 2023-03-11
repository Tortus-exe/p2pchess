// vim:foldmethod=marker:ft=rust
// IMPORTS {{{
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size, Clear, ClearType::All},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result, 
    cursor::{MoveTo, Show, Hide}
};
use std::{
    io::{stdout, Write},
};
use core::cmp::min;
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
pub mod inputbox;
use crate::pieces::chess_piece::chess_piece::Piece;
use crate::inputbox::inputbox::InputBox;
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

    let mut board_size_multiplier: usize = {
        let (x,y) = size()?;
        (min(x,y)/10).into()
    };

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
    let mut ibox = InputBox::new(7);

    board.request_move(&(1,1), &(1,3));

    print_board_grid(
        0, // x pos
        0,  // y pos
        8,  // num rows vertically
        8,  // num columns horizontally
        2*board_size_multiplier,  // width of each cell
        1*board_size_multiplier,  // height of each cell
        "ABCDEFGH".to_string(), 
        "12345678".to_string(), 
        Color::White, 
        Color::Black, 
        Color::Rgb{r:117, g:83, b:24})?;

    show_pieces(
        3+(board_size_multiplier as u16)/2, // x pos
        1+(board_size_multiplier as u16)/2,  // y pos
        8,  // num cols
        8,  // num rows
        2*board_size_multiplier,  // width of each cell
        1*board_size_multiplier,  // height of each cell
        &board, // the board
        Color::White,
        Color::Black
    )?;

    draw_input_box(52,0, &ibox)?;

    loop {
        execute!(stdout(), MoveTo(0, 0))?;
        let event=read()?;
        if let Event::Key(KeyEvent{code: c, modifiers: m, ..})=event {
            match (c, m) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => break,
                (KeyCode::Backspace, _) => drop(ibox.delete()),
                (KeyCode::Char(x), KeyModifiers::NONE) => drop(ibox.append(x)),
                (KeyCode::Char(x), KeyModifiers::SHIFT) => drop(ibox.append(x)),
                _ => ()
            }
        }
        if let Event::Resize(x,y)=event{
            execute!(stdout(), Clear(All))?;
            board_size_multiplier = (min(x,y)/10).into();
            print_board_grid(0, 0, 8, 8, 2*board_size_multiplier, 1*board_size_multiplier, "ABCDEFGH".to_string(), "12345678".to_string(), Color::White, Color::Black, Color::Rgb{r:117, g:83, b:24})?;
        }
        show_pieces(3+(board_size_multiplier as u16)/2, 1+(board_size_multiplier as u16)/2, 8, 8, 2*board_size_multiplier, 1*board_size_multiplier, &board, Color::White, Color::Black)?;
        draw_input_box(52,0, &ibox)?;
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
fn draw_input_box(x: u16, y: u16, i: &InputBox) -> Result<()> {
    execute!(stdout(),
        MoveTo(x,y),
        Print(format!("╭{:─<1$}╮", "", i.max_size)),
        MoveTo(x,y+1),
        Print(format!("│{: <1$}│", i.contents, i.max_size)),
        MoveTo(x,y+2),
        Print(format!("╰{:─<1$}╯", "", i.max_size))
    )?;
    stdout().flush()?;
    Ok(())
}
// }}}
// show_pieces {{{
fn show_pieces(x: u16, y: u16, num_cols: u8, num_rows: u8, column_width: usize, row_height: usize, board: &Board, color_a: Color, color_b: Color) -> Result<()> {
    for row in 0u8..num_rows as u8 {
        for col in 0..num_cols {
            // let piece = display_version_of.get(&board.state[row*num_cols+col]).unwrap_or(&board.state[row*num_cols+col]);
            let piece = if let Some(&c)=board.get_at(&(col, row)) {c.display_char()} else {' '};
            execute!(stdout(),
                MoveTo(x+(col as usize*column_width) as u16, y+(row as usize*row_height) as u16),
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
