pub mod king {
    pub use crate::pieces::chess_piece::chess_piece::{King, Piece, Square};
    use crate::board::Board;

    impl Piece for King {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self) -> bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn set_position(&mut self, &p: &Square) {self.pos = p;}
        fn can_move_to(&self, &(tx, ty): &Square, board: &Board) -> bool {
            if let Some(p)=board.get_at(&(tx,ty)) {
                let check = p.is_white()!=self.is_white && ((tx+1==self.pos.0 || self.pos.0+1==tx) || (ty+1==self.pos.1 || self.pos.1+1==ty));
                //self.has_moved |= check;
                return check;
            }
            return false;
        }
    }
}
