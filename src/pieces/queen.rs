pub mod queen {
    pub use crate::pieces::chess_piece::chess_piece::{Queen, Piece, Square};
    use crate::board::Board;

    impl Piece for Queen {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self) -> bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn can_move_to(&mut self, &(tx, ty): &Square, board: &Board) -> bool {
            let (mut px,mut py) = self.pos;
            while px < 8 && py < 8 {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.is_white!=p.is_white() {return true;}
                }
                if board.get_at(&(px,py))!=None && (px,py)!=self.pos {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px+1;
                py=py+1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.is_white!=p.is_white() {return true;}
                }
                if px==0 || py==0 || (board.get_at(&(px,py))!=None && (px,py)!=self.pos) {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px-1;
                py=py-1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && p.is_white()!=self.is_white {return true;}
                }
                if px==7 || py==0 || (board.get_at(&(px,py))!=None && (px,py)!=self.pos) {break;}
                if (px,py)==(tx,ty) {return true;}
                px=px+1;
                py=py-1;
            }
            (px,py) = self.pos;
            loop {
                if let Some(p)=board.get_at(&(px,py)) {
                    if (px,py)==(tx,ty) && self.is_white!=p.is_white() {return true;}
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
                    if p.is_white()!=self.is_white && (px,py)==(tx,ty){
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
                    if p.is_white()!=self.is_white && (px,py)==(tx,ty){
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
                    if p.is_white()!=self.is_white && (px,py)==(tx,ty){
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
                    if p.is_white()!=self.is_white && (px,py)==(tx,ty){
                        return true;
                    }
                }
                if board.get_at(&(px,py))!=None {break;}
            }
            return false;
        }
    }
}
