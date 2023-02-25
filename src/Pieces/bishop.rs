pub mod Bishop {
    pub use crate::Pieces::chessPiece::chessPiece::{Bishop, Piece, Square};
    use crate::board::Board;
    
    impl Piece for Bishop {
        fn isWhite(&self)->bool {self.isWhite}
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
            return false;
        }
    }
}
