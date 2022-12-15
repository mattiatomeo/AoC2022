#[derive(Debug)]
pub struct CharStack {
    data: Vec<char>,
}

impl CharStack {
    pub fn new() -> CharStack {
        CharStack { data: Vec::new() }
    }

    pub fn push(&mut self, character: char) {
        self.data.push(character);
    }

    pub fn pop(&mut self, character: char) -> char {
        match self.data.pop() {
            Some(value) => value,
            None => panic!("You called pop on an empty stack!")
        }
    }
}