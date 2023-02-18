mod chessPiece {
    pub type Square = (u8,u8);

    pub enum ChessPiece {
        Pawn(Pawn),
        Queen(Queen),
        King(King),
        Rook(Rook),
        Knight(Knight),
        Bishop(Bishop),
    }

    pub struct Pawn   {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}
    pub struct Queen  {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}
    pub struct King   {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}
    pub struct Rook   {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}
    pub struct Knight {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}
    pub struct Bishop {pos: Square, isWhite: bool, displayChar: char, hasMoved: bool}

    pub trait Piece {
        pub fn getPosition(&self) -> Square;
        pub fn canMoveTo(&self, pos: &Square, board: &Board) -> bool;
    }
}
