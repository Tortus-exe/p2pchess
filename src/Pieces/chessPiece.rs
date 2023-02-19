mod chessPiece {
    use crate::Board::Board;
    
    pub type Square = (u8,u8);

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ChessPiece {
        Pawn(Pawn),
        Queen(Queen),
        King(King),
        Rook(Rook),
        Knight(Knight),
        Bishop(Bishop),
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Pawn   {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Queen  {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct King   {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Rook   {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Knight {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Bishop {pub pos: Square, pub isWhite: bool, pub displayChar: char, pub hasMoved: bool}

    pub trait Piece {
        fn isWhite(&self) -> bool;
        fn getPosition(&self) -> Square;
        fn canMoveTo(&self, pos: &Square, board: &Board) -> bool;
    }
    
    impl Piece for ChessPiece {
        fn isWhite(&self) -> bool {
            match *self {
                Self::Pawn(p) => p.isWhite,
                Self::Queen(p) => p.isWhite,
                Self::King(p) => p.isWhite,
                Self::Rook(p) => p.isWhite,
                Self::Knight(p) => p.isWhite,
                Self::Bishop(p) => p.isWhite,
            }
        }
    
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
                Self::Pawn(p) =>   p.canMoveTo(s, b),
                Self::Queen(p) =>  p.canMoveTo(s, b),
                Self::King(p) =>   p.canMoveTo(s, b),
                Self::Rook(p) =>   p.canMoveTo(s, b),
                Self::Knight(p) => p.canMoveTo(s, b),
                Self::Bishop(p) => p.canMoveTo(s, b),
            }
        }
    }
}
