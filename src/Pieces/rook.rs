use crate::Pieces::chessPiece::{Rook, Piece, Square};

impl Piece for Rook {
    fn getPosition(&self) -> Square {self.pos}
    fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        let mut (cx,cy) = self.pos;
        while cy>=0 && board.get_at((cx,cy)) == None {
            if (cx,cy)==(tx,ty) {return true;}
            cy=cy-1;
        }
        (cx,cy)=self.pos;
        while cy<8 && board.get_at((cx,cy))==None {
            if (cx,cy)==(tx,ty) {return true;}
            cy=cy+1;
        }
        (cx,cy)=self.pos;
        while cx>=0 && board.get_at((cx,cy)) == None {
            if (cx,cx)==(tx,ty) {return true;}
            cx=cx-1;
        }
        (cx,cx)=self.pos;
        while cx<8 && board.get_at((cx,cy))==None {
            if (cx,cx)==(tx,ty) {return true;}
            cx=cx+1;
        }
        return false;
    }
}
