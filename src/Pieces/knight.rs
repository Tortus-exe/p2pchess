pub mod Knight {
    pub use crate::Pieces::chessPiece::chessPiece::{Knight, Piece, Square};
    use crate::board::Board;

    impl Piece for Knight {
        fn displayChar(&self)->char{self.displayChar}
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, _: &Board) -> bool {
            let possible_moves = vec![(tx+2, ty+1), (tx+2, ty-1), (tx-2, ty+1), (tx-2, ty-1), 
                                  (tx+1, ty+2), (tx-1, ty+2), (tx+1, ty-2), (tx-1, ty-2)];
            return possible_moves.contains(&self.pos);
        }
    }
}
