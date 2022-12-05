use std::collections::VecDeque;

#[derive(Clone)]
pub struct CargoStack {
    pub stack: VecDeque<char>,
}

impl CargoStack {
    pub fn new(init_contents: Vec<char>) -> CargoStack {
        CargoStack {
            stack: init_contents.into(),
        }
    }

    pub fn add(&mut self, containers: Vec<char>) {
        for container in containers {
            self.stack.push_front(container);
        }
    }

    pub fn remove(&mut self, amount: u16) -> Vec<char> {
        let mut ret = Vec::new();
        for _ in 0..amount {
            let moved_con = self.stack.pop_front();
            if moved_con.is_some() {
                ret.push(moved_con.unwrap());
            }
        }

        ret
    }

    pub fn add_in_order(&mut self, mut containers: Vec<char>) {
        containers.reverse();
        for container in containers {
            self.stack.push_front(container);
        }
    }

    pub fn get_top(&self) -> char {
        *self
            .stack
            .front()
            .expect("Expected the stack to not be empty")
    }
}
