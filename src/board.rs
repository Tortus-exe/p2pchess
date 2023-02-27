use std::collections::HashMap;
use crate::pieces::{
    chess_piece::chess_piece::{Square, ChessPiece, Piece},
    king::king::King,
    pawn::pawn::Pawn,
    knight::knight::Knight,
    queen::queen::Queen,
    bishop::bishop::Bishop,
    rook::rook::Rook
};

pub struct Board {
    pub state: HashMap<Square, ChessPiece>,
}

impl Board {
    fn insert_char_to_piece(key: char, loc: Square, disp: char) -> ChessPiece {
        match key {
            'P' => ChessPiece::Pawn(Pawn{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'B' => ChessPiece::Bishop(Bishop{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'N' => ChessPiece::Knight(Knight{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'Q' => ChessPiece::Queen(Queen{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'R' => ChessPiece::Rook(Rook{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'K' => ChessPiece::King(King{pos:loc, is_white:false,display_char:disp,has_moved:false,}),
            'p' => ChessPiece::Pawn(Pawn{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            'b' => ChessPiece::Bishop(Bishop{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            'n' => ChessPiece::Knight(Knight{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            'q' => ChessPiece::Queen(Queen{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            'r' => ChessPiece::Rook(Rook{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            'k' => ChessPiece::King(King{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
            _ => ChessPiece::Pawn(Pawn{pos:loc, is_white:true,display_char:disp,has_moved:false,}),
        }
    }

    pub fn new(&data: &[[char; 8]; 8]) -> Board{
        let mut b = Board {state: HashMap::new()};

        for (y,row) in data.iter().enumerate() {
            for (x,p) in row.iter().enumerate() {
                if *p != ' ' {
                    let rx = x.try_into().unwrap();
                    let ry = y.try_into().unwrap();
                    b.state.insert((rx,ry), Self::insert_char_to_piece(*p, (rx,ry), *p));
                }
            }
        }
        b
    }

    pub fn request_move(&mut self, &(fx, fy): &Square, &(tx,ty): &Square) -> bool {
        if let Some(&piece) = self.get_at(&(fx,fy)) {
            if piece.can_move_to(&(tx,ty), &self) {
                //let taken_piece = 
                self.state.insert((tx, ty), piece);
                self.state.remove(&(fx,fy));
            }
        }
        return false;
    }
    
    pub fn get_at(&self, sqr: &Square) -> Option<&ChessPiece> {
        return self.state.get(&sqr);
    }
}
