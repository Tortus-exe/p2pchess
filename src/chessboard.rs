struct Board {
    state: Vec<char>,
    /*    x ->
     * y 0 1 2 3 4 5 6 7
     * ↓ 1
     *   2
     *   3
     *   4
     *   5
     *   6
     *   7
     */
}

impl Board {
    pub fn new(data: Vec<char>) -> Board {
        Board {
            state: data,
        }
    }

    pub fn find(&self, piece: char) -> Vec<(u8, u8)>{
        let mut retval = Vec::new();
        self.state.iter().enumerate().for_each(|(i,&p)|
            if p==piece { retval.push((i as u8/8, i as u8%8)); }
        );
        retval
    }

    pub fn get_at(&self, (file, rank): (u8, u8)) -> char {
        self.state[(rank << 3 | file) as usize]
    }

    pub fn move_to(&mut self, (fromfile, fromrank): (u8, u8), (tofile, torank): (u8, u8)) {
        self.state[(torank << 3 | tofile) as usize] = self.get_at((fromfile, fromrank));
        self.state[(fromrank << 3 | fromfile) as usize] = ' ';
    }

    /*
     * checks whether or not there is a valid piece that can move to a target square.
     * 
     * param: kind -- lowercase = white, uppercase = black,
     *                P = pawn, B = bishop, R = rook, Q = queen, K = king, N = knight
     *
     * returns: a list of valid locations of those pieces.
     * 
     */
    fn gen_valid_origin_points( kind: char,
                         (tx, ty): (u8, u8), 
                       ) -> Vec<(u8, u8)> {
        Vec<_> possibilities = match kind {
            'p'       => vec![(tx, ty+2), (tx, ty+1), (tx+1, ty+1), (tx-1, ty+1)];
            'r' | 'R' => (0..=7).map(|x| (x, ty)).chain(
                         (0..=7).map(|y| (tx, y)))
                         .collect(); // turn this chaining into a macro
            'k' | 'K' => (-1..=1).map(|x| (-1..=1).map(|y| (tx+x, ty+y))).collect();
            _ => vec![];
        }
    }
}
