pub mod inputbox {
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
    }
}
