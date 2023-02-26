pub mod Rook {
    pub use crate::Pieces::chessPiece::chessPiece::{Rook, Piece, Square};
    use crate::board::Board;

    impl Piece for Rook {
        fn displayChar(&self)->char{self.displayChar}
        fn isWhite(&self) -> bool {self.isWhite}
        fn getPosition(&self) -> Square {self.pos}
        fn canMoveTo(&self, &(tx, ty): &Square, board: &Board) -> bool {
            let (mut px,mut py) = self.pos;
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
