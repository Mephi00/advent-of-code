use crate::third::rucksack::{self, Rucksack};

pub struct Group {
    pub identifier_prio: u32,
}

impl Group {
    pub fn new((rs_one, rs_two, rs_three): (Rucksack, Rucksack, Rucksack)) -> Group {
        let mut identifier_prio = 0;
        for item in rs_one.contents {
            if rs_two.contents.contains(&item) && rs_three.contents.contains(&item) {
                identifier_prio = rucksack::get_priority(item);
            }
        }

        Group {
            identifier_prio: identifier_prio,
        }
    }
}
