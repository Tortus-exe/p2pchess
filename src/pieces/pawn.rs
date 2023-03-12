pub mod pawn {
    pub use crate::pieces::chess_piece::chess_piece::{Pawn, Piece, Square};
    use crate::board::Board;
    
    impl Piece for Pawn {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self)->bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn set_position(&mut self, &p: &Square) {self.pos = p;}
        fn can_move_to(&self, &(tx, ty): &Square, board: &Board) -> bool {
            if board.get_at(&(tx, ty)) == None {
                if self.is_white {
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
                Some(&p) => p.is_white(),
                None    => !self.is_white,
            };
            if self.is_white {
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
