pub mod knight {
    pub use crate::pieces::chess_piece::chess_piece::{Knight, Piece, Square};
    use crate::board::Board;

    impl Piece for Knight {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self) -> bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn set_position(&mut self, &p: &Square) {self.pos = p;}
        fn can_move_to(&self, &(tx, ty): &Square, _: &Board) -> bool {
            let possible_moves = vec![
                (tx+2, ty+1),
                (tx+1, ty+2),
                (tx+2, ty.checked_sub(1).unwrap_or(10)), 
                (tx.checked_sub(2).unwrap_or(10), ty+1), 
                (tx.checked_sub(2).unwrap_or(10), ty.checked_sub(1).unwrap_or(10)), 
                (tx.checked_sub(1).unwrap_or(10), ty+2), 
                (tx+1, ty.checked_sub(2).unwrap_or(10)),
                (tx.checked_sub(1).unwrap_or(10), ty.checked_sub(2).unwrap_or(10))];
            return possible_moves.contains(&self.pos);
        }
    }
}
