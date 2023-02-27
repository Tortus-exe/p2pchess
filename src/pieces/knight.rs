pub mod knight {
    pub use crate::pieces::chess_piece::chess_piece::{Knight, Piece, Square};
    use crate::board::Board;

    impl Piece for Knight {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self) -> bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn can_move_to(&mut self, &(tx, ty): &Square, _: &Board) -> bool {
            let possible_moves = vec![(tx+2, ty+1), (tx+2, ty-1), (tx-2, ty+1), (tx-2, ty-1), 
                                  (tx+1, ty+2), (tx-1, ty+2), (tx+1, ty-2), (tx-1, ty-2)];
            return possible_moves.contains(&self.pos);
        }
    }
}
