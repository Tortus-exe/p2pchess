mod Pawn {
    pub use crate::Pieces::chessPiece::{Pawn, Piece, Square};
    use crate::Board::Board;
    
    impl Piece for Pawn {
        fn isWhite(&self)->bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, board: &Board) -> bool {
            if board.get_at(&(tx, ty)) == None {
                if self.isWhite {
                    return  ty < 7 &&
                            tx == self.pos.0 &&
                            (ty+1==self.pos.1 ||
                            (ty==4 && self.pos.1==6 && board.get_at(&(tx,3))==None && board.get_at(&(tx,4))==None));
                } else {
                    return  ty > 0 && ty < 8 &&
                            tx == self.pos.0 &&
                            ((ty-1==self.pos.1 && board.get_at(&(tx,ty))==None) || 
                            (ty==3 && self.pos.1==1 && board.get_at(&(tx,2))==None && board.get_at(&(tx,3))==None));
                }
            }
            //white and taking
            let target_piece_is_white: bool = match board.get_at(&(tx,ty)) {
                Some(&p) => p.isWhite(),
                None    => !self.isWhite,
            };
            if self.isWhite {
                return  ty < 7 &&
                        (tx == self.pos.0+1 || 
                         tx+1==self.pos.0) &&
                        ty+1== self.pos.1 &&
                        !target_piece_is_white;
            } else {
                return  ty > 0 && ty < 8 &&
                        (tx == self.pos.0+1 ||
                         tx+1==self.pos.0) &&
                        ty==self.pos.1+1 &&
                        target_piece_is_white;
            }
        }
    }
}
