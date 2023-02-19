use crate::Pieces::chessPiece::{Queen, Piece, Square};

impl Piece for Queen {
    fn getPosition(&self) -> Square {self.pos}
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
        while py>=0 && board.get_at((px,py)) == None {
            if (px,py)==(tx,ty) {return true;}
            py=py-1;
        }
        (px,py)=self.pos;
        while py<8 && board.get_at((px,py))==None {
            if (px,py)==(tx,ty) {return true;}
            py=py+1;
        }
        (px,py)=self.pos;
        while px>=0 && board.get_at((px,py)) == None {
            if (px,px)==(tx,ty) {return true;}
            px=px-1;
        }
        (px,px)=self.pos;
        while px<8 && board.get_at((px,py))==None {
            if (px,px)==(tx,ty) {return true;}
            px=px+1;
        }
        return false;

    }
}
