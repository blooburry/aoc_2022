#[derive(Debug)]
pub struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    pub fn new () -> Self {
        CrateStack {
            crates: Vec::new(),
        }
    }

    pub fn build_stack(&mut self, c: char){
        self.crates.insert(0, c);
    }

    pub fn add_crate(&mut self, c: char) {
        self.crates.push(c);
    }

    pub fn remove_crate(&mut self) -> Option<char> {
        self.crates.pop()
    }

    pub fn get_last_crate(&self) -> Option<&char> {
        self.crates.last()
    }
}