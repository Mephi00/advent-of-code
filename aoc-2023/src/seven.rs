pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> usize {
    let mut total = 0;

    let mut hands: Vec<Hand> = input_str.lines().map(|line| Hand::new(line)).collect();

    hands.sort();

    for (idx, hand) in hands.iter().enumerate() {
        total += (idx + 1) * hand.bet;
    }

    total
}

fn part_two(input_str: &String) -> usize {
    let mut total = 0;

    let mut hands: Vec<Hand> = input_str
        .lines()
        .map(|line| Hand::new_part_two(line))
        .collect();

    hands.sort();

    for (idx, hand) in hands.iter().enumerate() {
        println!("{:?}", hand);
        total += (idx + 1) * hand.bet;
    }

    total
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    constellation: Constellation,
    pub bet: usize,
    values: Vec<u8>,
}

impl Hand {
    pub fn new(hand_str: &str) -> Self {
        let hands_fun: Vec<&str> = hand_str
            .split_whitespace()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                return Some(s.trim());
            })
            .collect();
        let chars: Vec<char> = hands_fun[0].chars().collect();
        let mut values: Vec<u8> = Vec::new();
        let mut constellation = Constellation::High;
        let mut curr_pair = ' ';
        for ch in hands_fun[0].chars() {
            values.push(
                match ch.to_digit(10) {
                    Some(dig) => dig,
                    None => match ch {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("the char {ch} of {hand_str} doesn't match any card"),
                    },
                }
                .try_into()
                .unwrap(),
            );

            match chars.iter().fold(0, |state, c| {
                if *c == ch {
                    return state + 1;
                }

                state
            }) {
                2 => {
                    if constellation == Constellation::OnePair && curr_pair != ch {
                        constellation = Constellation::TwoPair;
                    } else if constellation == Constellation::Triple {
                        constellation = Constellation::FullHouse;
                    } else if constellation < Constellation::OnePair {
                        constellation = Constellation::OnePair;
                        curr_pair = ch;
                    }
                }
                3 => {
                    if constellation == Constellation::OnePair {
                        constellation = Constellation::FullHouse;
                    } else if constellation < Constellation::Triple {
                        constellation = Constellation::Triple;
                    }
                }
                4 => constellation = Constellation::Four,
                5 => constellation = Constellation::Five,
                _ => {}
            }
        }

        Self {
            constellation: constellation,
            bet: hands_fun[1].parse().unwrap(),
            values,
        }
    }

    pub fn new_part_two(hand_str: &str) -> Self {
        let hands_fun: Vec<&str> = hand_str
            .split_whitespace()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                return Some(s.trim());
            })
            .collect();
        let chars: Vec<char> = hands_fun[0].chars().collect();
        let mut values: Vec<u8> = Vec::new();
        let mut joker_consts = Vec::new();
        let count_joker = chars.iter().fold(0, |state, c| {
            if *c == 'J' {
                return state + 1;
            }

            state
        });
        for ch in hands_fun[0].chars() {
            values.push(
                match ch.to_digit(10) {
                    Some(dig) => dig,
                    None => match ch {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("the char {ch} of {hand_str} doesn't match any card"),
                    },
                }
                .try_into()
                .unwrap(),
            );

            if ch == 'J' {
                continue;
            }

            let mut constellation = Constellation::High;
            let mut curr_pair = ' ';
            for ch_i in hands_fun[0].chars() {
                if ch_i == 'J' {
                    continue;
                }
                let mut num_occ = chars.iter().fold(0, |state, c| {
                    if *c == ch_i {
                        return state + 1;
                    }

                    state
                });

                if ch_i == ch {
                    num_occ += count_joker;
                }

                match num_occ {
                    2 => {
                        if constellation == Constellation::OnePair && curr_pair != ch_i {
                            constellation = Constellation::TwoPair;
                        } else if constellation == Constellation::Triple {
                            constellation = Constellation::FullHouse;
                        } else if constellation < Constellation::OnePair {
                            constellation = Constellation::OnePair;
                            curr_pair = ch_i;
                        }
                    }
                    3 => {
                        if constellation == Constellation::OnePair {
                            constellation = Constellation::FullHouse;
                        } else if constellation < Constellation::Triple {
                            constellation = Constellation::Triple;
                            curr_pair = ' ';
                        }
                    }
                    4 => constellation = Constellation::Four,
                    5 => constellation = Constellation::Five,
                    _ => {}
                }
            }

            joker_consts.push(constellation);
        }

        joker_consts.sort();
        if joker_consts.is_empty() {
            joker_consts.push(Constellation::Five);
        }
        Self {
            constellation: joker_consts.last().unwrap().clone(),
            bet: hands_fun[1].parse().unwrap(),
            values,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.constellation == other.constellation {
            for i in 0..self.values.len() {
                if self.values[i] != other.values[i] {
                    return self.values[i].partial_cmp(&other.values[i]);
                }
            }
        }

        self.constellation.partial_cmp(&other.constellation)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.constellation == other.constellation {
            for i in 0..self.values.len() {
                if self.values[i] != other.values[i] {
                    return self.values[i].cmp(&other.values[i]);
                }
            }
        }

        self.constellation.cmp(&other.constellation)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
enum Constellation {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Triple = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}
