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
                         (kx, ky): (u8, u8), 
                       ) -> impl Iterator<Item=(u8, u8)> {
        let genVec: Vec<(i8, i8)>;
        let tx = kx as i8;
        let ty = ky as i8;
        match kind {
            'p'       => genVec = vec![(tx, ty+2), (tx, ty+1), (tx+1, ty+1), (tx-1, ty+1)],
            'P'       => genVec = vec![(tx, ty-2), (tx, ty-1), (tx+1, tx-1), (tx-1, ty-1)],
            'r' | 'R' => genVec = (0..=7).map(|x| (x, ty)).chain(
                                  (0..=7).map(|y| (tx, y)))
                                  .collect(),
            'k' | 'K' => genVec = (0..=2).flat_map(move |x| (0..=2).map(move |y| (tx+x-1, ty+y-1))).collect(),
            'b' | 'B' => genVec = (0..=7).map(|x| {
                                      if tx > ty {(x-ty+tx, x)} 
                                      else {(x, x+ty-tx)}}).chain(
                                  (0..=7).map(|x| (tx+ty-x, x)))
                                  .collect(),
            'q' | 'Q' => genVec = (0..=7).map(|x| (x, ty)).chain(
                                  (0..=7).map(|y| (tx, y)).chain(
                                  (0..=7).map(|x| (tx+ty-x, x)).chain(
                                  (0..=7).map(|x| {
                                      if tx > ty {(x-ty+tx, x)} 
                                      else {(x, x+ty-tx)}}))))
                                  .collect(),
            'n' | 'N' => genVec = vec![(tx+2, ty+1), (tx+2, ty-1), (tx-2, ty+1), (tx-2, ty-1), 
                                       (tx+1, ty+2), (tx-1, ty+2), (tx+1, ty-2), (tx-1, ty-2)],
            _ => genVec = vec![],
        }
        genVec.into_iter().filter(move |(x,y)| *x>=0 && *y>=0 && (*x,*y)!=(tx,ty)).map(|(x,y)| (x as u8, y as u8))
    }
}
