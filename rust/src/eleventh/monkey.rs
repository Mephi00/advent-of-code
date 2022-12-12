use num_bigint::{BigUint, ToBigUint};

pub struct Monkey {
    items: Vec<BigUint>,
    operation: Box<dyn Fn(&BigUint) -> BigUint>,
    pub test_num: u128,
    pub test_true: usize,
    pub test_false: usize,
    pub num_inspected_items: u128,
}

impl Monkey {
    pub fn new(
        starting_items: Vec<BigUint>,
        operation: Box<dyn Fn(&BigUint) -> BigUint>,
        test_num: u128,
        test_true: usize,
        test_false: usize,
    ) -> Monkey {
        Monkey {
            items: starting_items,
            operation: operation,
            test_num: test_num,
            test_false: test_false,
            test_true: test_true,
            num_inspected_items: 0,
        }
    }

    pub fn receive_item(&mut self, item: BigUint) {
        self.items.push(item);
    }

    pub fn turn(
        &mut self,
        monkey_true: &mut Self,
        monkey_false: Option<&mut Self>,
        min_divisor: u128,
    ) {
        let different_monkeys = monkey_false.as_ref().is_some();
        if different_monkeys {
            let false_monkey = monkey_false.unwrap();

            for item in &self.items {
                self.num_inspected_items += 1;
                let mut new_worry = (self.operation)(item) / 3.to_biguint().unwrap();

                new_worry %= min_divisor;

                if &new_worry % self.test_num == 0.to_biguint().unwrap() {
                    monkey_true.receive_item(new_worry);
                } else {
                    false_monkey.receive_item(new_worry);
                }
            }
        } else {
            for item in &self.items {
                self.num_inspected_items += 1;
                let new_worry = (self.operation)(item) / 3.to_biguint().unwrap();

                monkey_true.receive_item(new_worry);
            }
        }
        self.items = Vec::new();
    }

    pub fn turn_without_reduction(
        &mut self,
        monkey_true: &mut Self,
        monkey_false: Option<&mut Self>,
        min_divisor: u128,
    ) {
        // let different_monkeys = monkey_false.as_ref().is_some();
        // if different_monkeys {
        let false_monkey = monkey_false.unwrap();

        for item in &self.items {
            self.num_inspected_items += 1;
            let mut new_worry = (self.operation)(item);

            new_worry %= min_divisor;
            if &new_worry % self.test_num == 0.to_biguint().unwrap() {
                monkey_true.receive_item(new_worry);
            } else {
                false_monkey.receive_item(new_worry);
            }
        }
        // } else {
        //     for item in &self.items {
        //         self.num_inspected_items += 1;
        //         let mut new_worry = (self.operation)(item);

        //         // new_worry %= min_divisor;

        //         monkey_true.receive_item(new_worry);
        //     }
        // }
        self.items = Vec::new();
    }
}
