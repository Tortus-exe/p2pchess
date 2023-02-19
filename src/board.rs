use std::collections::HashMap;
use crate::chessPiece::{Square, ChessPiece};

mod Board {
    pub struct Board {
        pub state: HashMap<Square, ChessPiece>,
    }

    impl Board {
        fn requestMove(&self, &(tx, ty): Square) -> bool;
        //TODO
    }
}
