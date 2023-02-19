use crate::Pieces::chessPiece::{Bishop, Piece, Square};

impl Piece for Bishop {
    fn getPosition(&self) -> Square {}
    fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        let mut (px,py) = self.pos;
        while px < 8 && py < 8 && board.get_at((px,py))==None{
            if (px,py)==(tx,ty) {return true;}
            px=px+1;
            py=py+1;
        }
        (px,py) = self.pos;
        loop {
            if (px,py)==(tx,ty) {return true;}
            if px==0 || py==0 || board.get_at((px,py))!=None {break;}
            px=px-1;
            py=py-1;
        }
        (px,py) = self.pos;
        loop {
            if (px,py)==(tx,ty) {return true;}
            if px==7 || py==0 || board.get_at((px,py))!=None {break;}
            px=px+1;
            py=py-1;
        }
        (px,py) = self.pos;
        loop {
            if (px,py)==(tx,ty) {return true;}
            if px==0 || py==7 || board.get_at((px,py))!=None {break;}
            px=px-1;
            py=py+1;
        }
        return false;
    }
}
