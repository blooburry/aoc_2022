use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Clone)]
pub struct Tree {
    position: (u32, u32),
    height: u32,
    pub scenic_score: u32,
}

impl Tree {
    pub fn new(position: (u32, u32), height: u32) -> Self {
        Self { position, height, scenic_score: 0 }
    }
    pub fn position(&self) -> (u32, u32) {
        self.position
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl PartialOrd<Self> for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}