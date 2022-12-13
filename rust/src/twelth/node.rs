#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub shortest_path: usize,
    pub position: (usize, usize),
    pub height: u8,
    visited: bool,
}

impl Node {
    pub fn new(pos: (usize, usize), height: u8) -> Node {
        Node {
            shortest_path: usize::MAX,
            position: pos,
            height: height,
            visited: false,
        }
    }

    pub fn new_start(pos: (usize, usize)) -> Node {
        Node {
            shortest_path: 0,
            position: pos,
            height: 'a' as u8,
            visited: false,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn visited(&self) -> bool {
        self.visited
    }
}
