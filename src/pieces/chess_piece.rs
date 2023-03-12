pub mod chess_piece {
    use crate::board::Board;
    
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
    pub struct Pawn   {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Queen  {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct King   {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Rook   {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Knight {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Bishop {pub pos: Square, pub is_white: bool, pub display_char: char, pub has_moved: bool}

    pub trait Piece {
        fn display_char(&self) -> char;
        fn is_white(&self) -> bool;
        fn get_position(&self) -> Square;
        fn set_position(&mut self, pos: &Square);
        fn can_move_to(&self, pos: &Square, board: &Board) -> bool;
    }
    
    impl Piece for ChessPiece {
        fn display_char(&self) -> char {
            match *self {
                Self::Pawn(p) => p.display_char,
                Self::Queen(p) => p.display_char,
                Self::King(p) => p.display_char,
                Self::Rook(p) => p.display_char,
                Self::Knight(p) => p.display_char,
                Self::Bishop(p) => p.display_char,
            }
        }

        fn is_white(&self) -> bool {
            match *self {
                Self::Pawn(p) => p.is_white,
                Self::Queen(p) => p.is_white,
                Self::King(p) => p.is_white,
                Self::Rook(p) => p.is_white,
                Self::Knight(p) => p.is_white,
                Self::Bishop(p) => p.is_white,
            }
        }
    
        fn get_position(&self) -> Square {
            match *self {
                Self::Pawn(p) => p.get_position(),
                Self::Queen(p) => p.get_position(),
                Self::King(p) => p.get_position(),
                Self::Rook(p) => p.get_position(),
                Self::Knight(p) => p.get_position(),
                Self::Bishop(p) => p.get_position(),
            }
        }

        fn set_position(&mut self, k: &Square) {
            match *self {
                Self::Pawn(mut p) => p.set_position(k),
                Self::Queen(mut p) => p.set_position(k),
                Self::King(mut p) => p.set_position(k),
                Self::Rook(mut p) => p.set_position(k),
                Self::Knight(mut p) => p.pos = *k,
                Self::Bishop(mut p) => p.set_position(k),
            }

            // if let Self::Knight(mut p) = *self {
            //     p.pos = *k;
            //     println!("{:?}", p.pos);
            // }

            // if let Self::Knight(mut p) = *self {
            //     println!("{:?}", p.pos);
            // }
        }
        
        fn can_move_to(&self, s: &Square, b: &Board) -> bool {
            match self {
                Self::Pawn(p) =>   p.can_move_to(s, b),
                Self::Queen(p) =>  p.can_move_to(s, b),
                Self::King(p) =>   p.can_move_to(s, b),
                Self::Rook(p) =>   p.can_move_to(s, b),
                Self::Knight(p) => p.can_move_to(s, b),
                Self::Bishop(p) => p.can_move_to(s, b),
            }
        }
    }
}
