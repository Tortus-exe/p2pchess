use crate::Pieces::chessPiece::{Knight, Piece, Square};

impl Piece for Knight {
    fn getPosition(&self) -> Square {self.pos}
    fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        ix = tx as i8;
        iy = ty as i8;
        ipos = self.pos as (i8,i8);
        possible_moves = vec![(tx+2, ty+1), (tx+2, ty-1), (tx-2, ty+1), (tx-2, ty-1), 
                              (tx+1, ty+2), (tx-1, ty+2), (tx+1, ty-2), (tx-1, ty-2)];
        return possible_moves.contains(ipos);
    }
}
