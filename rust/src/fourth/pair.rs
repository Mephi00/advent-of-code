use crate::fourth::elf::Elf;

#[derive(Debug, Clone)]
pub struct Pair {
    elf_one: Elf,
    elf_two: Elf,
}

impl Pair {
    pub fn new(input: &str) -> Pair {
        let elfs_str: Vec<&str> = input.split(",").collect();

        let elf_one = Elf::new(elfs_str[0]);
        let elf_two = Elf::new(elfs_str[1]);

        Pair {
            elf_one: elf_one,
            elf_two: elf_two,
        }
    }

    pub fn complete_overlapping(&self) -> bool {
        if self.elf_one.lower_barrier <= self.elf_two.lower_barrier
            && self.elf_one.upper_barrier >= self.elf_two.upper_barrier
        {
            true
        } else if self.elf_one.lower_barrier >= self.elf_two.lower_barrier
            && self.elf_one.upper_barrier <= self.elf_two.upper_barrier
        {
            true
        } else {
            false
        }
    }

    pub fn partly_overlapping(&self) -> bool {
        if self.complete_overlapping() {
            true
        } else if self.elf_one.lower_barrier <= self.elf_two.upper_barrier
            && !(self.elf_one.upper_barrier < self.elf_two.lower_barrier)
        {
            true
        } else if self.elf_one.upper_barrier >= self.elf_two.lower_barrier
            && !(self.elf_one.lower_barrier > self.elf_two.upper_barrier)
        {
            true
        } else {
            false
        }
    }
}
