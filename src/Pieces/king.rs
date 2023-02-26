pub mod King {
    pub use crate::Pieces::chessPiece::chessPiece::{King, Piece, Square};
    use crate::board::Board;

    impl Piece for King {
        fn displayChar(&self)->char{self.displayChar}
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, board: &Board) -> bool {
            if let Some(p)=board.get_at(&(tx,ty)) {
                return p.isWhite()!=self.isWhite && ((tx+1==self.pos.0 || self.pos.0+1==tx) || (ty+1==self.pos.1 || self.pos.1+1==ty));
            }
            return false;
        }
    }
}
