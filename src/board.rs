use hashbrown::HashMap;
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
        if let Some(&mut mut piece) = self.state.get_mut(&(fx,fy)) {
            if piece.can_move_to(&(tx,ty), self) {
                //let taken_piece = 
                self.state.insert((tx, ty), piece);
                self.state.remove(&(fx,fy));
            }
        }
        return false;
    }

    pub fn request_castle_queenside(&mut self, is_white: bool) -> bool {
        let y = if is_white {0} else {7};
        let mut move_signal = false;

        if self.state.contains_key(&(0,y)) &&
           !self.state.contains_key(&(1,y)) &&
           !self.state.contains_key(&(2,y)) &&
           !self.state.contains_key(&(3,y)) &&
           self.state.contains_key(&(4,y))
        {
            if let Some([ChessPiece::King(king), ChessPiece::Rook(rook)]) = self.state.get_many_mut([&(4,y), &(0,y)]) {
                if  !king.has_moved && !rook.has_moved {
                    move_signal = true;
                }
            }
        }

        if move_signal {
            if let Some(k) = self.state.remove(&(4,y)){
                self.state.insert((2,y), k);
            }
            if let Some(r) = self.state.remove(&(0,y)) {
                self.state.insert((3,y), r);
            }
        }
        false
    }
    
    pub fn get_at(&self, sqr: &Square) -> Option<&ChessPiece> {
        return self.state.get(&*sqr);
    }
}
