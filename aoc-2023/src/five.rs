use std::{sync::Arc, thread};

pub fn exec(input_str: String) {
    println!("part one: {}", part_one(&input_str));
    println!("part two: {}", part_two(&input_str));
}

fn part_one(input_str: &String) -> i64 {
    let maps = Arc::new(create_map_collection(input_str));
    create_seeds(
        maps,
        input_str
            .lines()
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap()),
    )
    .min()
    .unwrap()
}

fn part_two(input_str: &String) -> i64 {
    let parsed_seed_ranges: Vec<i64> = input_str
        .lines()
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut start = None;

    let mut seeds = Vec::new();

    for entry in parsed_seed_ranges {
        if start.is_none() {
            start = Some(entry);
        } else {
            let start_un = start.unwrap();
            let mut vec = (start_un..start_un + entry).collect();
            seeds.append(&mut vec);
            start = None;
        }
    }

    let mappings = Arc::new(create_map_collection(input_str));

    let num_threads = 16;
    let len_slice = seeds.len() / num_threads;

    let mut slices = Vec::new();

    for i in 0..num_threads {
        slices.push(&seeds[len_slice * i..len_slice * (i + 1)]);
    }

    println!("starting multithreadding");
    let mut lowest = i64::MAX;
    thread::scope(|scope| {
        let mut locations: Vec<thread::ScopedJoinHandle<i64>> = Vec::new();
        for slice in slices {
            locations.push(scope.spawn(|| {
                create_seeds(mappings.clone(), slice.iter().map(|f| *f))
                    .min()
                    .unwrap()
            }));
        }

        while !locations.iter().all(|thr| thr.is_finished()) {}

        for location in locations {
            let result = location.join().unwrap();

            println!("local low: {result}");

            if result < lowest {
                lowest = result;
            }
        }
    });

    lowest
}

fn build_mapping(input_str: &str) -> Mapping {
    let mut ret = Vec::new();

    for line in input_str.lines() {
        let splits: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();

        ret.push(MappingEntry {
            start_dest: splits[0],
            start_source: splits[1],
            range: splits[2],
        });
    }

    Mapping::new(ret)
}

fn create_map_collection(input_str: &String) -> MappingCollection {
    let mut seed_to_soil = Mapping::default();
    let mut soil_to_fertilizer = Mapping::default();
    let mut fertilizer_to_water = Mapping::default();
    let mut water_to_light = Mapping::default();
    let mut light_to_temperature = Mapping::default();
    let mut temperature_to_humidity = Mapping::default();
    let mut humidity_to_location = Mapping::default();

    let mut temp_map = Mapping::default();
    'outer: for split in input_str.rsplit(":").flat_map(|s| s.trim().rsplit("\n\n")) {
        if split.ends_with("map") {
            match split.split_whitespace().next() {
                Some(map_name) => match map_name {
                    "seed-to-soil" => {
                        seed_to_soil = temp_map;
                        break 'outer;
                    }
                    "soil-to-fertilizer" => soil_to_fertilizer = temp_map,
                    "fertilizer-to-water" => fertilizer_to_water = temp_map,
                    "water-to-light" => water_to_light = temp_map,
                    "light-to-temperature" => light_to_temperature = temp_map,
                    "temperature-to-humidity" => temperature_to_humidity = temp_map,
                    "humidity-to-location" => humidity_to_location = temp_map,
                    _ => {}
                },
                None => {}
            }
            temp_map = Mapping::default();
        } else {
            temp_map = build_mapping(split.trim());
        }
    }

    MappingCollection {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn create_seeds(
    maps: Arc<MappingCollection>,
    seeds: impl Iterator<Item = i64>,
) -> impl Iterator<Item = i64> {
    seeds.map(move |id| {
        maps.humidity_to_location.find_mapping(
            maps.temperature_to_humidity.find_mapping(
                maps.light_to_temperature.find_mapping(
                    maps.water_to_light.find_mapping(
                        maps.fertilizer_to_water.find_mapping(
                            maps.soil_to_fertilizer
                                .find_mapping(maps.seed_to_soil.find_mapping(id)),
                        ),
                    ),
                ),
            ),
        )
    })
}

struct MappingCollection {
    pub seed_to_soil: Mapping,
    pub soil_to_fertilizer: Mapping,
    pub fertilizer_to_water: Mapping,
    pub water_to_light: Mapping,
    pub light_to_temperature: Mapping,
    pub temperature_to_humidity: Mapping,
    pub humidity_to_location: Mapping,
}

unsafe impl Send for MappingCollection {}

unsafe impl Sync for MappingCollection {}

#[derive(Default, Debug)]
struct Mapping {
    entries: Vec<MappingEntry>,
}

impl Mapping {
    pub fn new(entries: Vec<MappingEntry>) -> Self {
        Mapping { entries }
    }

    pub fn find_mapping(&self, index: i64) -> i64 {
        for entry in &self.entries {
            if let Some(id) = entry.find(index) {
                return id;
            }
        }

        return index;
    }
}

#[derive(Debug)]
struct MappingEntry {
    pub start_dest: i64,
    pub start_source: i64,
    pub range: i64,
}

impl MappingEntry {
    pub fn find(&self, index: i64) -> Option<i64> {
        if index >= self.start_source && index < self.start_source + self.range {
            return Some(index - self.start_source + self.start_dest);
        }

        None
    }
}
