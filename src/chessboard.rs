pub struct Board {
    pub state: Vec<char>,
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

    pub fn get_at(&self, (file, rank): (u8,u8)) -> char {
        self.state[(rank << 3 | file) as usize]
    }

    pub fn move_to(&mut self, 
                    piece: char, 
                    is_blacks_turn: bool,
                    file_discriminator: Option<u8>,
                    rank_discriminator: Option<u8>,
                    (tofile, torank): (u8, u8)) -> bool{
        let arg_piece: char = if is_blacks_turn { piece.to_ascii_uppercase() } else { piece };
        let (mut currx, mut curry) : (u8, u8) = (8, 8);
        let filehint: u8 = file_discriminator.unwrap_or(8);
        let rankhint: u8 = rank_discriminator.unwrap_or(8);
        let mut times_found: u8 = 0;

        for (sqx, sqy) in Self::valid_origin_points(self, arg_piece, (tofile, torank)) {
            if self.state[(sqy << 3 | sqx) as usize] == arg_piece {
                match times_found {
                    0 => (currx, curry) = (sqx, sqy),
                    1 => (currx, curry) = if filehint == currx { (sqx, sqy) } 
                                          else { (8, 8) },
                    2 => (currx, curry) = if filehint == currx && rankhint == curry { (sqx, sqy) }
                                          else { (8, 8) },
                    _ => (currx, curry) = (8, 8), // <- Note: this should NEVER HAPPEN!!
                }
                times_found = times_found + 1;
            }
        }
        if (currx, curry) != (8, 8) {
            self.state[(torank << 3 | tofile) as usize] = self.get_at((currx, curry));
            self.state[(curry << 3 | currx) as usize] = ' ';
        }
        return (currx, curry) != (8, 8);
    }

    /*
     * generates all valid origin points of a valid piece that can move to a target square.
     * 
     * param: kind -- lowercase = white, uppercase = black,
     *                P = pawn, B = bishop, R = rook, Q = queen, K = king, N = knight
     *
     * returns: a list of valid locations of those pieces.
     * 
     */
    fn valid_origin_points(&mut self, 
                            kind: char,
                        (kx, ky): (u8, u8), 
                       ) -> impl Iterator<Item=(u8, u8)> {
        let gen_vec: Vec<(i8, i8)>;
        let tx = kx as i8;
        let ty = ky as i8;
        match kind {
            'p'       => gen_vec = if self.get_at((kx, ky))==' ' {vec![(tx, ty+2), (tx, ty+1)]} else {vec![(tx+1, ty+1), (tx-1, ty+1)]},
            'P'       => gen_vec = if self.get_at((kx, ky))==' ' {vec![(tx, ty-2), (tx, ty-1)]} else {vec![(tx+1, ty-1), (tx-1, ty-1)]},
            'r' | 'R' => gen_vec = (0..=7).map(|x| (x, ty)).chain(
                                  (0..=7).map(|y| (tx, y)))
                                  .collect(),
            'k' | 'K' => gen_vec = (0..=2).flat_map(move |x| (0..=2).map(move |y| (tx+x-1, ty+y-1))).collect(),
            'b' | 'B' => gen_vec = (0..=7).map(|x| {
                                      if tx > ty {(x-ty+tx, x)} 
                                      else {(x, x+ty-tx)}}).chain(
                                  (0..=7).map(|x| (tx+ty-x, x)))
                                  .collect(),
            'q' | 'Q' => gen_vec = (0..=7).map(|x| (x, ty)).chain(
                                  (0..=7).map(|y| (tx, y)).chain(
                                  (0..=7).map(|x| (tx+ty-x, x)).chain(
                                  (0..=7).map(|x| {
                                      if tx > ty {(x-ty+tx, x)} 
                                      else {(x, x+ty-tx)}}))))
                                  .collect(),
            'n' | 'N' => gen_vec = vec![(tx+2, ty+1), (tx+2, ty-1), (tx-2, ty+1), (tx-2, ty-1), 
                                       (tx+1, ty+2), (tx-1, ty+2), (tx+1, ty-2), (tx-1, ty-2)],
            _ => gen_vec = vec![],
        }
        gen_vec.into_iter()
               .filter(move |(x,y)| *x>=0 && 
                                    *y>=0 && 
                                    *x<=7 && 
                                    *y<=7 && 
                                    (*x,*y)!=(tx,ty))
               .map(|(x,y)| (x as u8, y as u8))
    }
}
