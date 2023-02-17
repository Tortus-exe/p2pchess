type Square = Square;

struct pawn   {pos: Square, isWhite: bool, id: u8, displayChar: char}
struct queen  {pos: Square, isWhite: bool, id: u8, displayChar: char}
struct king   {pos: Square, isWhite: bool, id: u8, displayChar: char}
struct rook   {pos: Square, isWhite: bool, id: u8, displayChar: char}
struct knight {pos: Square, isWhite: bool, id: u8, displayChar: char}
struct bishop {pos: Square, isWhite: bool, id: u8, displayChar: char}

pub trait chessPiece {
    pub fn getPosition(&self) -> Square;
    pub fn requestMoveTo(&self, &board: Board) -> bool;
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}


impl chessPiece for pawn {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {
        
    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square> {
        if self.isWhite {
            if board.get_at(self.pos).id == 0 {
                //copy from old code;
            }
        }
    }
}
impl chessPiece for queen {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {

    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}
impl chessPiece for king {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {

    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}
impl chessPiece for rook {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {

    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}
impl chessPiece for knight {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {

    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}
impl chessPiece for bishop {
    fn getPosition(&self) -> Square {self.pos}
    fn requestMoveTo(&self, &board: Board) -> bool {

    }
    pub fn validMoves(&self, &board: Board) -> Iterator<Item=Square>
}
