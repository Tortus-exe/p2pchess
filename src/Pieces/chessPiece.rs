mod chessPiece {
    use crate::Board::Board;
    
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
        fn getPosition(&self) -> Square;
        fn canMoveTo(&self, pos: &Square, board: &Board) -> bool;
    }
    
    impl Piece for ChessPiece {
        fn getPosition(&self) -> Square {
            match *self {
                Self::Pawn(p) => p.getPosition(),
                Self::Queen(p) => p.getPosition(),
                Self::King(p) => p.getPosition(),
                Self::Rook(p) => p.getPosition(),
                Self::Knight(p) => p.getPosition(),
                Self::Bishop(p) => p.getPosition(),
            }
        }
        
        fn canMoveTo(&self, s: &Square, b: &Board) -> bool {
            match self {
                Self::Pawn(p)   => p.canMoveTo(s, b),
                Self::Queen(p)  => p.canMoveTo(s, b),
                Self::King(p)   => p.canMoveTo(s, b),
                Self::Rook(p)   => p.canMoveTo(s, b),
                Self::Knight(p) => p.canMoveTo(s, b),
                Self::Bishop(p) => p.canMoveTo(s, b),
            }
        }
    }
}
