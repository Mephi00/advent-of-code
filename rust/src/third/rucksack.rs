#[derive(Clone)]
pub struct Rucksack {
    pub contents: Vec<char>,
    pub priority_total: u32,
}

impl Rucksack {
    pub fn new(contents: &str) -> Rucksack {
        let chars = contents.char_indices();
        let bag_two_barrier = chars.clone().count() / 2;

        let mut bag_one = Vec::new();
        let mut prio_total = 0;

        for (index, item) in chars {
            if index < bag_two_barrier {
                bag_one.push(item);
            } else {
                if bag_one.contains(&item) {
                    prio_total = get_priority(item);
                }
            }
        }

        Rucksack {
            contents: contents.chars().collect(),
            priority_total: prio_total,
        }
    }
}

pub fn get_priority(char: char) -> u32 {
    if char.is_uppercase() {
        ((char as u8) - 38).into()
    } else {
        ((char as u8) - 96).into()
    }
}
