pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i32 {
    let mut total = 0;
    'outer: for line in input_str.lines() {
        let game = create_game(line);
        for pull in game.pulls {
            if !pull.possible() {
                continue 'outer;
            }
        }

        total += game.id;
    }

    total
}

fn part_two(input_str: &String) -> i64 {
    let mut total: i64 = 0;

    for line in input_str.lines() {
        let mut curr_lowest_red: i64 = 1;
        let mut curr_lowest_green: i64 = 1;
        let mut curr_lowest_blue: i64 = 1;
        let game = create_game(line);
        for pull in game.pulls {
            if pull.red > 0 && curr_lowest_red < pull.red.into() {
                curr_lowest_red = pull.red.into();
            }

            if pull.blue > 0 && curr_lowest_blue < pull.blue.into() {
                curr_lowest_blue = pull.blue.into();
            }

            if pull.green > 0 && curr_lowest_green < pull.green.into() {
                curr_lowest_green = pull.green.into();
            }
        }

        let power = curr_lowest_blue * curr_lowest_green * curr_lowest_red;

        total += power;
    }

    total
}

fn create_game(line: &str) -> Game {
    let mut ret = vec![];
    let mut split = line.split(": ").map(|f| f.trim());
    let game = split.next().unwrap();

    for pull_str in split.next().unwrap().split(";").map(|f| f.trim()) {
        ret.push(GamePull::create(pull_str));
    }

    Game {
        pulls: ret,
        id: game
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap(),
    }
}

struct Game {
    pub pulls: Vec<GamePull>,
    pub id: i32,
}

#[derive(Debug)]
struct GamePull {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl GamePull {
    pub fn create(input: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for split in input.split(",") {
            let mut comp = split.split_whitespace();
            let num = comp.next().unwrap().parse::<i32>().unwrap();
            match comp.next().unwrap() {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                color => println!("color: {} couldn't be matched", color),
            };
        }

        Self { red, blue, green }
    }

    pub fn possible(&self) -> bool {
        self.blue <= 14 && self.red <= 12 && self.green <= 13
    }
}
