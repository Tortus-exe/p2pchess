pub mod Queen {
    pub use crate::Pieces::chessPiece::chessPiece::{Queen, Piece, Square};
    use crate::board::Board;

    impl Piece for Queen {
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, board: &Board) -> bool {
            let (mut px,mut py) = self.pos;
            while px < 8 && py < 8 {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.isWhite!=p.isWhite() {return true;}
                }
                if board.get_at(&(px,py))!=None && (px,py)!=self.pos {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px+1;
                py=py+1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.isWhite!=p.isWhite() {return true;}
                }
                if px==0 || py==0 || (board.get_at(&(px,py))!=None && (px,py)!=self.pos) {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px-1;
                py=py-1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && p.isWhite()!=self.isWhite {return true;}
                }
                if px==7 || py==0 || (board.get_at(&(px,py))!=None && (px,py)!=self.pos) {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px+1;
                py=py-1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.isWhite!=p.isWhite() {return true;}
                }
                if px==0 || py==7 || (board.get_at(&(px,py))!=None && (px,py)!=self.pos) {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px-1;
                py=py+1;
            }
            (px,py)=self.pos;
            while py>0 {
                if (px,py)==(tx,ty) {return true;}
                py=py-1;
                if let Some(p)=board.get_at(&(px,py)) {
                    if p.isWhite()!=self.isWhite && (px,py)==(tx,ty){
                        return true;
                    }
                }
                if board.get_at(&(px,py))!=None {break;}
            }
            (px,py)=self.pos;
            while py<8 {
                if (px,py)==(tx,ty) {return true;}
                py=py+1;
                if let Some(p)=board.get_at(&(px,py)) {
                    if p.isWhite()!=self.isWhite && (px,py)==(tx,ty){
                        return true;
                    }
                }
                if board.get_at(&(px,py))!=None {break;}
            }
            (px,py)=self.pos;
            while px>0 {
                if (px,px)==(tx,ty) {return true;}
                px=px-1;
                if let Some(p)=board.get_at(&(px,py)) {
                    if p.isWhite()!=self.isWhite && (px,py)==(tx,ty){
                        return true;
                    }
                }
                if board.get_at(&(px,py))!=None {break;}
            }
            (px,py)=self.pos;
            while px<8 {
                if (px,px)==(tx,ty) {return true;}
                px=px+1;
                if let Some(p)=board.get_at(&(px,py)) {
                    if p.isWhite()!=self.isWhite && (px,py)==(tx,ty){
                        return true;
                    }
                }
                if board.get_at(&(px,py))!=None {break;}
            }
            return false;
        }
    }
}
