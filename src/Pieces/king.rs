mod King {
    pub use crate::Pieces::chessPiece::{King, Piece, Square};
    use crate::Board::Board;

    impl Piece for King {
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, _: &Board) -> bool {
            return (tx+1==self.pos.0 || self.pos.0+1==tx) || (ty+1==self.pos.1 || self.pos.1+1==ty);
        }
    }
}
