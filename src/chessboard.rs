struct Board {
    state: Vec<char>,
}

impl Board {
    fn new(data: Vec<char>) -> Board {
        Board {
            state: data,
        }
    }

    fn find(&self, piece: char) -> Vec<(u8, u8)>{
        let mut retval = Vec::new();
        self.state.iter().enumerate().for_each(|(i,&p)|
            if p==piece { retval.push((i as u8/8, i as u8%8)); }
        );
        retval
    }

    fn get_at(&self, (file, rank): (u8, u8)) -> char {
        self.state[(rank*8+file) as usize]
    }

    fn move_to(&mut self, (fromfile, fromrank): (u8, u8), (tofile, torank): (u8, u8)) {
        self.state[(torank*8+tofile) as usize] = self.get_at((fromfile, fromrank));
        self.state[(fromrank*8+fromfile) as usize] = ' ';
    }
}
