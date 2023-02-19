use crate::Pieces::chessPiece::{Pawn, Piece, Square};
mod Pawn {
    impl Piece for Pawn {
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
            if board.get_at((tx, ty)) == None {
                if self.isWhite {
                    return( ty < 7 &&
                            tx == self.pos.0 &&
                            ((ty+1==self.pos.1 && board.get_at((tx,ty))==None) ||
                            (ty==4 && self.pos.1==6 && board.get_at((tx,3))==None && board.get_at((tx,4))==None)));
                } else {
                    return( ty > 0 && ty < 8 &&
                            tx == self.pos.0 &&
                            ((ty-1==self.pos.1 && board.get_at((tx,ty))==None) || 
                            (ty==3 && self.pos.1==1 && board.get_at((tx,2))==None && board.get_at((tx,3))==None)));
                }
            }
            //white and taking
            if self.isWhite {
                return( ty < 7 &&
                        (tx == self.pos.0+1 || 
                         tx+1==self.pos.0) &&
                        ty+1== self.pos.1 &&
                        !board.get_at((tx,ty)).isWhite);
            } else {
                return( ty > 0 && ty < 8 &&
                        (tx == self.pos.0+1 ||
                         tx+1==self.pos.0) &&
                        ty==self.pos.1+1 &&
                        board.get_at((tx,ty)).isWhite);
            }
        }
    }
}
