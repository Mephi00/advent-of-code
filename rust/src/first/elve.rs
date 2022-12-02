#[derive(Debug)]
pub struct Elve {
    calories: Vec<u32>,
    pub total: u32,
}

impl Elve {
    pub fn new(calories: Vec<u32>) -> Elve {
        Elve {
            total: calories.clone().iter().sum(),
            calories: calories,
        }
    }
}

impl PartialEq for Elve {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories && self.total == other.total
    }

    fn ne(&self, other: &Self) -> bool {
        self.total != other.total
    }
}

impl Eq for Elve {}

impl PartialOrd for Elve {
    fn ge(&self, other: &Self) -> bool {
        self.total >= other.total
    }

    fn gt(&self, other: &Self) -> bool {
        self.total > other.total
    }

    fn le(&self, other: &Self) -> bool {
        self.total <= other.total
    }

    fn lt(&self, other: &Self) -> bool {
        self.total < other.total
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.total > other.total {
            return Some(std::cmp::Ordering::Greater);
        }

        if self.total < other.total {
            return Some(std::cmp::Ordering::Less);
        }

        if self.total == other.total {
            return Some(std::cmp::Ordering::Equal);
        }

        None
    }
}

impl Ord for Elve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
