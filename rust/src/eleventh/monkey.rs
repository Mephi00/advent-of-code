pub struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test_num: u32,
    pub test_true: usize,
    pub test_false: usize,
    pub num_inspected_items: u64,
}

impl Monkey {
    pub fn new(
        starting_items: Vec<u32>,
        operation: Box<dyn Fn(u32) -> u32>,
        test_num: u32,
        test_true: usize,
        test_false: usize,
    ) -> Monkey {
        Monkey {
            items: starting_items,
            operation: Box::new(operation),
            test_num: test_num,
            test_false: test_false,
            test_true: test_true,
            num_inspected_items: 0,
        }
    }

    pub fn receive_item(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn turn(&mut self, monkey_true: &mut Self, monkey_false: &mut Self) {
        for item in &self.items {
            self.num_inspected_items += 1;
            let new_worry = (self.operation)(*item) / 3;

            if new_worry % self.test_num == 0 {
                monkey_true.receive_item(new_worry);
            } else {
                monkey_false.receive_item(new_worry);
            }
        }

        self.items = Vec::new();
    }
}
