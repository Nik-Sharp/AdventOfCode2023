use core::panic;
use std::fs;

extern crate itertools;
use itertools::Itertools;

extern crate rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

extern crate tokio;

#[derive(Clone, Debug)]
struct Map {
    pub source_range: (u128, u128),
    pub add_amount: i64,
}

impl From<&str> for Map {
    fn from(string: &str) -> Map {
        let mut parameters = string.split_ascii_whitespace();

        let destination: u128 = parameters.next().unwrap().parse().unwrap();
        let source: u128 = parameters.next().unwrap().parse().unwrap();
        let range: u128 = parameters.next().unwrap().parse().unwrap();

        Map {
            source_range: (source, source + range - 1),
            add_amount: (destination as i64 - source as i64),
        }
    }
}

struct Almanac {
    pub parsing_state: ParserState,

    pub seeds: Vec<(u128, u128)>,
    pub seed_to_soil: Vec<Map>,
    pub soil_to_fertilizer: Vec<Map>,
    pub fertilizer_to_water: Vec<Map>,
    pub water_to_light: Vec<Map>,
    pub light_to_temperature: Vec<Map>,
    pub temperature_to_humidity: Vec<Map>,
    pub humidity_to_location: Vec<Map>,
}

impl Almanac {
    fn new() -> Almanac {
        Almanac {
            parsing_state: ParserState::Seeds,
            seeds: vec![],
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        }
    }
}
enum ParserState {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl From<&str> for ParserState {
    fn from(string: &str) -> ParserState {
        match string.trim() {
            "seed-to-soil map:" => ParserState::SeedToSoil,
            "soil-to-fertilizer map:" => ParserState::SoilToFertilizer,
            "fertilizer-to-water map:" => ParserState::FertilizerToWater,
            "water-to-light map:" => ParserState::WaterToLight,
            "light-to-temperature map:" => ParserState::LightToTemperature,
            "temperature-to-humidity map:" => ParserState::TemperatureToHumidity,
            "humidity-to-location map:" => ParserState::HumidityToLocation,
            _ => panic!("Invalid string for ParserState: {}", string),
        }
    }
}
fn parse_input(almanac: &mut Almanac, line: &str) {
    if line.get(0..=4).unwrap_or("N/A") == "seeds" {
        let mut split = line.split(":");
        split.next();

        almanac.seeds = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .enumerate()
            .group_by(|(i, _x)| i / 2)
            .into_iter()
            .map(|(_i, mut x)| {
                (
                    x.next().unwrap().1.parse::<u128>().unwrap(),
                    x.next().unwrap().1.parse::<u128>().unwrap(),
                )
            })
            .collect();

        return;
    }

    if line.is_empty() {
        return;
    }

    if line.chars().next().unwrap().is_alphabetic() {
        almanac.parsing_state = ParserState::from(line);
    } else {
        let map = Map::from(line);

        let maps = match almanac.parsing_state {
            ParserState::Seeds => panic!("Unreachable"),
            ParserState::SeedToSoil => &mut almanac.seed_to_soil,
            ParserState::SoilToFertilizer => &mut almanac.soil_to_fertilizer,
            ParserState::FertilizerToWater => &mut almanac.fertilizer_to_water,
            ParserState::WaterToLight => &mut almanac.water_to_light,
            ParserState::LightToTemperature => &mut almanac.light_to_temperature,
            ParserState::TemperatureToHumidity => &mut almanac.temperature_to_humidity,
            ParserState::HumidityToLocation => &mut almanac.humidity_to_location,
        };

        maps.push(map);
    }
}

// (value, if changed, (acceptable devation from value))
fn convert_with_map(val: u128, map: &Map) -> (u128, bool, (u128, u128)) {
    if val >= map.source_range.0 && val <= map.source_range.1 {
        return (
            (val as i64 + map.add_amount) as u128,
            true,
            map.source_range,
        );
    }

    (val, false, (val, val))
}
fn get_location_from_start(almanac: &Almanac, seed: u128) -> (u128, (u128, u128)) {
    let conversions = [
        &almanac.seed_to_soil,
        &almanac.soil_to_fertilizer,
        &almanac.fertilizer_to_water,
        &almanac.water_to_light,
        &almanac.light_to_temperature,
        &almanac.temperature_to_humidity,
        &almanac.humidity_to_location,
    ];

    let mut value = seed;
    let mut range: (u128, u128) = (u128::MAX, u128::MAX);

    for conversion in conversions {
        for map in conversion {
            let old_value = value;
            let result = convert_with_map(value, map);
            value = result.0;

            if result.1 {
                range = (
                    range.0.min(old_value - result.2 .0),
                    range.1.min(result.2 .1 - old_value),
                );
                break;
            }
        }
    }

    (value, range)
}

fn get_min(almanac: &Almanac, range: (u128, u128)) -> u128 {
    let mut min = u128::MAX;

    let mut index: u128 = range.0;

    while index < (range.0 + range.1) {
        let result = get_location_from_start(&almanac, index);

        min = min.min(result.0);

        index += result.1 .1;

        index += 1;
    }

    min
}

fn main() {
    // Gets input from file
    let full_input;
    let mut almanac = Almanac::new();

    match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input
                .split("\n")
                .for_each(|x| parse_input(&mut almanac, x))
        }
        Err(x) => panic!("Error in reading, {}", x),
    };

    let min = almanac.seeds.par_iter().map(|range| {
        let min = get_min(&almanac, *range);
        min
    });

    println!("{:?}", min.min().unwrap());
}
