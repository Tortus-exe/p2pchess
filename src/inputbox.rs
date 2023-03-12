pub mod inputbox {
    use crate::board::Board;

    pub struct InputBox {
        pub contents: String,
        pub max_size: usize
    }

    impl InputBox {
        pub fn new(max: usize) -> Self {
            InputBox {contents: String::from(""), max_size: max}
        }

        pub fn append(&mut self, c: char) -> bool {
            if self.contents.len() < self.max_size { 
                self.contents.push(c);
                return true;
            }
            false
        }

        pub fn delete(&mut self) -> bool {
            matches!(self.contents.pop(), Some(_))
        }

        pub fn request_to_board(&mut self, board: &mut Board) {
            match self.contents.to_lowercase().as_str() {
                "o-o-o" => board.request_castle_queenside(false),
                "o-o" => board.request_castle_kingside(false),
                req => {
                    let reqchars: Vec<char> = req.chars().collect();
                    let mut c = 0;
                    let piece = if "abcdefgh".contains(reqchars[0]) {
                        'p'
                    } else {
                        c=c+1;
                        reqchars[0]
                    };
                    //check taking
                    let y = "abcdefgh".find(reqchars[c]).unwrap() as u8;
                    c=c+1;
                    let x = reqchars[c].to_digit(10).unwrap() as u8 -1;
                    board.request_move(piece, &(x,y))
                }
            };
            self.contents = String::from("");
        }
    }
}
