use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct MapType {
    map_type: String,
}

#[derive(Debug)]
struct AgricultureMap {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}

fn string_to_int_set(string: &str) -> HashSet<u64> {
    let mut res: HashSet<u64> = HashSet::new();
    let numbers = string
        .trim()
        .split(" ")
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.trim().parse::<u64>().unwrap());

    for number in numbers {
        res.insert(number);
    }
    return res;
}

fn parse_maps(contents: &str) -> (MapType, Vec<AgricultureMap>) {
    //println!(">>>{}", contents);
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let map_type = lines[0]
        .trim()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();

    let mut res = Vec::new();

    for line in lines.iter().skip(1) {
        if line.trim().len() == 0 {
            continue;
        }
        let mut parts = line.split_whitespace();
        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();
        let map_type = map_type.clone();
        let map = AgricultureMap {
            dest_start,
            source_start,
            range_length,
        };
        //  println!("{:?}", map);
        res.push(map);
    }
    return (MapType { map_type }, res);
}

fn find_next_value(current_value: u64, maps: &Vec<AgricultureMap>) -> u64 {
    let mut res = current_value;
    //println!(
    //"Searching for next location for current_value ({})",
    //current_value
    //);
    for map in maps {
        //println!(
        //"{} {} {:?}",
        //map.source_start,
        //current_value,
        //map.source_start >= current_value
        //);
        if current_value >= map.source_start && current_value <= map.source_start + map.range_length
        {
            res = map.dest_start + current_value - map.source_start;
            break;
        }
    }
    //println!(
    //"Found next location for current_value ({}): {}",
    //current_value, res
    //);
    return res;
}

fn find_min_location(
    seeds: &HashSet<u64>,
    seed_to_soil_maps: Vec<AgricultureMap>,
    soil_to_fertilizer_maps: Vec<AgricultureMap>,
    fertilizer_to_water_maps: Vec<AgricultureMap>,
    water_to_light_maps: Vec<AgricultureMap>,
    light_to_temperature_maps: Vec<AgricultureMap>,
    temperature_to_humidity_maps: Vec<AgricultureMap>,
    humidity_to_location_maps: Vec<AgricultureMap>,
) -> u64 {
    let mut min = u64::max_value();
    for seed in seeds {
        let soil = find_next_value(*seed, &seed_to_soil_maps);
        let fertilizer = find_next_value(soil, &soil_to_fertilizer_maps);
        let water = find_next_value(fertilizer, &fertilizer_to_water_maps);
        let light = find_next_value(water, &water_to_light_maps);
        let temperature = find_next_value(light, &light_to_temperature_maps);
        let humidity = find_next_value(temperature, &temperature_to_humidity_maps);
        let location = find_next_value(humidity, &humidity_to_location_maps);
        println!(">>>> Found location for seed {}: {}", seed, location);
        if location < min {
            min = location;
        }
    }
    return min;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let contents = fs::read_to_string(file).unwrap();

    let mut seeds: HashSet<u64> = HashSet::new();

    let map_multi_line_regex = Regex::new(r"(?s)([-a-z]+) map:.(\d+ \d+ \d+\n)+").unwrap();
    let seeds_regex = Regex::new(r"seeds: (.*)").unwrap();

    let seeds_part = seeds_regex.captures(contents.as_str()).unwrap();
    let seeds_str = seeds_part.get(1).unwrap();
    seeds.extend(string_to_int_set(seeds_str.as_str()));
    // seeds.insert(13);

    let mut seed_to_soil_maps: Vec<AgricultureMap> = Vec::new();
    let mut soil_to_fertilizer_maps: Vec<AgricultureMap> = Vec::new();
    let mut fertilizer_to_water_maps: Vec<AgricultureMap> = Vec::new();
    let mut water_to_light_maps: Vec<AgricultureMap> = Vec::new();
    let mut light_to_temperature_maps: Vec<AgricultureMap> = Vec::new();
    let mut temperature_to_humidity_maps: Vec<AgricultureMap> = Vec::new();
    let mut humidity_to_location_maps: Vec<AgricultureMap> = Vec::new();

    for cap in map_multi_line_regex.find_iter(contents.as_str()) {
        let (map_type, map) = parse_maps(cap.as_str());
        match map_type.map_type.as_str() {
            "seed-to-soil" => seed_to_soil_maps.extend(map),
            "soil-to-fertilizer" => soil_to_fertilizer_maps.extend(map),
            "fertilizer-to-water" => fertilizer_to_water_maps.extend(map),
            "water-to-light" => water_to_light_maps.extend(map),
            "light-to-temperature" => light_to_temperature_maps.extend(map),
            "temperature-to-humidity" => temperature_to_humidity_maps.extend(map),
            "humidity-to-location" => humidity_to_location_maps.extend(map),
            _ => println!("Unknown map type: {}", map_type.map_type.as_str()),
        }
    }

    //println!("Seeds: {:?}", seeds);
    //println!("Seed-to-soil: {:?}", seed_to_soil_maps);
    //println!("Soil-to-fertilizer: {:?}", soil_to_fertilizer_maps);
    //println!("Fertilizer-to-water: {:?}", fertilizer_to_water_maps);
    //println!("Water-to-light: {:?}", water_to_light_maps);
    //println!("Light-to-temperature: {:?}", light_to_temperature_maps);
    //println!(
    //"Temperature-to-humidity: {:?}",
    //temperature_to_humidity_maps
    //);
    //println!("Humidity-to-location: {:?}", humidity_to_location_maps);

    let min_location = find_min_location(
        &seeds,
        seed_to_soil_maps,
        soil_to_fertilizer_maps,
        fertilizer_to_water_maps,
        water_to_light_maps,
        light_to_temperature_maps,
        temperature_to_humidity_maps,
        humidity_to_location_maps,
    );
    println!("Min location: {}", min_location);
}
