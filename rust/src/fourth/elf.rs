#[derive(Debug, Clone)]
pub struct Elf {
    pub lower_barrier: i32,
    pub upper_barrier: i32,
}

impl Elf {
    pub fn new(input: &str) -> Elf {
        let split: Vec<i32> = input
            .split("-")
            .map(|x| x.parse().expect("couldnt parse input as number"))
            .collect();

        Elf {
            lower_barrier: split[0],
            upper_barrier: split[1],
        }
    }
}
