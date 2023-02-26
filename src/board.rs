// mod chessPiece;
// mod Pieces {
//     mod chessPiece;
//     mod king;
//     mod pawn;
//     mod knight;
//     mod queen;
//     mod bishop;
//     mod rook;
// }


use std::collections::HashMap;
use crate::Pieces::{
    chessPiece::chessPiece::{Square, ChessPiece, Piece},
    king::King::King,
    pawn::Pawn::Pawn,
    knight::Knight::Knight,
    queen::Queen::Queen,
    bishop::Bishop::Bishop,
    rook::Rook::Rook
};

pub struct Board {
    pub state: HashMap<Square, ChessPiece>,
}

impl Board {
    // pub fn new() -> Board {
    //     Board {
    //         state: HashMap::new(),
    //     }
    // }

    fn insertCharToPiece(key: char, loc: Square, disp: char) -> ChessPiece {
        match key {
            'P' => ChessPiece::Pawn(Pawn{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'B' => ChessPiece::Bishop(Bishop{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'N' => ChessPiece::Knight(Knight{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'Q' => ChessPiece::Queen(Queen{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'R' => ChessPiece::Rook(Rook{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'K' => ChessPiece::King(King{pos:loc, isWhite:false,displayChar:disp,hasMoved:false,}),
            'p' => ChessPiece::Pawn(Pawn{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            'b' => ChessPiece::Bishop(Bishop{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            'n' => ChessPiece::Knight(Knight{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            'q' => ChessPiece::Queen(Queen{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            'r' => ChessPiece::Rook(Rook{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            'k' => ChessPiece::King(King{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
            _ => ChessPiece::Pawn(Pawn{pos:loc, isWhite:true,displayChar:disp,hasMoved:false,}),
        }
    }

    pub fn new(&data: &[[char; 8]; 8]) -> Board{
        let mut b = Board {state: HashMap::new()};

        for (y,row) in data.iter().enumerate() {
            for (x,p) in row.iter().enumerate() {
                if *p != ' ' {
                    let rx = x.try_into().unwrap();
                    let ry = y.try_into().unwrap();
                    b.state.insert((rx,ry), Self::insertCharToPiece(*p, (rx,ry), *p));
                }
            }
        }
        b
    }

    pub fn requestMove(&mut self, &(fx, fy): &Square, &(tx,ty): &Square) -> bool {
        if let Some(&piece) = self.get_at(&(fx,fy)) {
            if piece.canMoveTo(&(tx,ty), &self) {
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
