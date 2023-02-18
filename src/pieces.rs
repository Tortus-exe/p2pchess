type Square = (u8,u8);

struct pawn   {pos: Square, isWhite: bool, displayChar: char}
struct queen  {pos: Square, isWhite: bool, displayChar: char}
struct king   {pos: Square, isWhite: bool, displayChar: char}
struct rook   {pos: Square, isWhite: bool, displayChar: char}
struct knight {pos: Square, isWhite: bool, displayChar: char}
struct bishop {pos: Square, isWhite: bool, displayChar: char}

pub trait chessPiece {
    pub fn getPosition(&self) -> Square;
    pub fn requestMoveTo(&self, pos: Square, board: &Board) -> bool;
    pub fn canMoveTo(&self, pos: &Square, board: &Board) -> bool;
}

impl chessPiece for pawn {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        true
    }
    fn canMoveTo(&self, &(tx, ty): &Square, &board: &Board) -> bool {
        if board.get_at((tx, ty)) == None {
            if self.isWhite {
                return( ty < 7 &&
                        tx == self.pos.0 &&
                        (ty+1 == self.pos.1 ||
                        (ty==4 && self.pos.1==6)));
            } else {
                return( ty > 0 && ty < 8 &&
                        tx == self.pos.0 &&
                        (ty-1 == self.pos.1 || 
                        (ty==3 && self.pos.1==1)));
            }
        }
        //white and taking
        if self.isWhite {
            return( ty < 7 &&
                    (tx == self.pos.0+1 || 
                     tx+1==self.pos.0) &&
                    ty+1== self.pos.1);
        } else {
            return( ty > 0 && ty < 8 &&
                    (tx == self.pos.0+1 ||
                     tx+1==self.pos.0) &&
                    ty==self.pos.1+1);
        }
    }
}
