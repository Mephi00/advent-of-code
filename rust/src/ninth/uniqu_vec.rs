pub struct UniqueVec<T: PartialEq>(Vec<T>);

impl<T: PartialEq> UniqueVec<T> {
    pub fn new() -> UniqueVec<T> {
        UniqueVec(Vec::new())
    }

    pub fn push(&mut self, item: T) {
        if !self.0.contains(&item) {
            self.0.push(item);
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
