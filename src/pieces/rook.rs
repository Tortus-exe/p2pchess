pub mod rook {
    pub use crate::pieces::chess_piece::chess_piece::{Rook, Piece, Square};
    use crate::board::Board;

    impl Piece for Rook {
        fn display_char(&self)->char{self.display_char}
        fn is_white(&self) -> bool {self.is_white}
        fn get_position(&self) -> Square {self.pos}
        fn set_position(&mut self, &p: &Square) {self.pos = p;}
        fn can_move_to(&self, &(tx, ty): &Square, board: &Board) -> bool {
            let (mut px,mut py) = self.pos;
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
