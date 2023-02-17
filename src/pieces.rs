struct pawn {pos: (u8, u8), isWhite: bool,}
struct queen {pos: (u8, u8),isWhite: bool,}
struct king {pos: (u8, u8), isWhite: bool,}
struct rook {pos: (u8, u8), isWhite: bool,}
struct knight {pos: (u8, u8),isWhite: bool,}
struct bishop {pos: (u8, u8),isWhite: bool,}

pub trait chessPiece {
    pub fn getPosition(&self) -> (u8, u8);
    pub fn requestMoveTo(&self, &board) -> bool;
}


impl chessPiece for pawn {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {
        
    }
}
impl chessPiece for queen {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {

    }
}
impl chessPiece for king {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {

    }
}
impl chessPiece for rook {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {

    }
}
impl chessPiece for knight {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {

    }
}
impl chessPiece for bishop {
    fn getPosition(&self) -> (u8, u8) {self.pos}
    fn requestMoveTo(&self, &board) -> bool {

    }
}
