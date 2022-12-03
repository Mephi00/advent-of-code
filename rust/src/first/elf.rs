#[derive(Debug)]
pub struct Elf {
    calories: Vec<u32>,
    pub total: u32,
}

impl Elf {
    pub fn new(calories: Vec<u32>) -> Elf {
        Elf {
            total: calories.clone().iter().sum(),
            calories: calories,
        }
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories && self.total == other.total
    }

    fn ne(&self, other: &Self) -> bool {
        self.total != other.total
    }
}

impl Eq for Elf {}

impl PartialOrd for Elf {
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

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
