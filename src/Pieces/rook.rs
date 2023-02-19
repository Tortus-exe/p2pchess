mod Rook {
    pub use crate::Pieces::chessPiece::{Rook, Piece, Square};
    use crate::Board::Board;

    impl Piece for Rook {
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, board: &Board) -> bool {
            let (mut cx,mut cy) = self.pos;
            while board.get_at(&(cx,cy)) == None {
                if (cx,cy)==(tx,ty) {return true;}
                cy=cy-1;
            }
            (cx,cy)=self.pos;
            while cy<8 && board.get_at(&(cx,cy))==None {
                if (cx,cy)==(tx,ty) {return true;}
                cy=cy+1;
            }
            (cx,cy)=self.pos;
            while board.get_at(&(cx,cy)) == None {
                if (cx,cx)==(tx,ty) {return true;}
                cx=cx-1;
            }
            (cx,cy)=self.pos;
            while cx<8 && board.get_at(&(cx,cy))==None {
                if (cx,cx)==(tx,ty) {return true;}
                cx=cx+1;
            }
            return false;
        }
    }
}
