use crate::Pieces::chessPiece::{King, Piece, Square};

impl Piece for King {
    fn getPosition(&self) -> Square {self.pos}
    fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        return (tx+1==self.pos.0 || self.pos.0+1==tx) || (ty+1==self.pos.1 || self.pos.1+1==ty);
    }
}
