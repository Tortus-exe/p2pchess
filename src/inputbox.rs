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
                    if reqchars.len() < 2 { return; }
                    let mut c = 0;
                    let piece = if "abcdefgh".contains(reqchars[0]) && "1234567890x".contains(reqchars[1]) {
                        'p'
                    } else {
                        c=c+1;
                        reqchars[0]
                    };
                    let taking = 'x'==reqchars[c+1];
                    if taking { c=c+2; }

                    let x = "abcdefgh".find(reqchars[c]).unwrap_or(9) as u8;
                    c=c+1;
                    let mut y = reqchars[c].to_digit(10).unwrap_or(9) as u8;
                    if x>8||y>8 { return; }
                    y=8-y;
                    if taking == board.state.contains_key(&(x,y)) {
                        board.request_move(piece, &(x,y));
                    }
                    true
                }
            };
            self.contents = String::from("");
        }
    }
}
