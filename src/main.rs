use std::{collections::HashMap, fs, num::ParseIntError};

#[derive(Clone, Debug)]
struct Range {
    src_offset_start: i64,
    length: i64,
    dest_offset: i64,
}

impl Range {
    fn new(dest_pos: i64, src_pos: i64, length: i64) -> Range {
        Range {
            src_offset_start: src_pos,
            length,
            dest_offset: dest_pos - src_pos,
        }
    }

    fn is_in_src_range(&self, src_value: i64) -> bool {
        src_value >= self.src_offset_start && src_value < self.src_offset_start + self.length
    }

    fn convert(&self, value: i64) -> i64 {
        if !self.is_in_src_range(value) {
            return value;
        }
        value + self.dest_offset
    }
}

#[derive(Clone, Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn try_from(lines: Vec<&String>) -> Result<Self, &'static str> {
        let names_line = lines
            .first()
            .ok_or("Too few lines provided")?
            .strip_suffix(" map:")
            .expect("Failed to strip suffix");

        println!("{}", names_line);

        let names: Vec<&str> = names_line.trim().split("-to-").collect();

        if names.len() != 2 {
            return Err("Invalid mapping names length");
        }

        let ranges: Result<Vec<Range>, &'static str> = lines
            .iter()
            .skip(1)
            .map(|line| -> Result<Range, &'static str> {
                let range_params: Result<Vec<i64>, ParseIntError> =
                    line.split_whitespace().map(str::parse::<i64>).collect();

                let range_params = range_params.or(Err("Failed to parse range"))?;
                if range_params.len() != 3 {}

                Ok(Range::new(
                    range_params[0],
                    range_params[1],
                    range_params[2],
                ))
            })
            .collect();

        let ranges = ranges.map_err(|_| "Failed to parse range")?;

        Ok(Map {
            from: names[0].to_string(),
            to: names[1].to_string(),
            ranges,
        })
    }

    fn convert(&self, src_value: i64) -> i64 {
        // println!("Converting {} from {} to {}", src_value, self.from, self.to);
        let mut dest_value = src_value;
        for range in &self.ranges {
            if range.is_in_src_range(src_value) {
                dest_value = range.convert(src_value);
            }
        }

        dest_value
    }
}

/// The `get_input` function reads the contents of a file at the given path and returns them as a vector of strings, with
/// leading and trailing whitespace removed from each line.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `get_input` function is a `String` that represents the file path from which you
/// want to read the input.
///
/// Returns:
///
/// The function `get_input` returns a `Vec<String>`, which is a vector of strings.
fn read_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn print_map(map: &Map) {
    println!("{}-to-{}", map.from, map.to);
    // for range in &map.ranges {
    //     println!(
    //         "{} {} {}",
    //         range.src_offset_start, range.length, range.dest_offset
    //     );
    // }
    println!()
}

fn read_maps(lines: &[String]) -> Result<Vec<Map>, Box<dyn std::error::Error>> {
    let mut maps: Vec<Map> = Vec::new();
    let mut current_chunk: Vec<&String> = Vec::new();

    for line in lines.iter().skip(2) {
        if line.is_empty() {
            maps.push(Map::try_from(current_chunk.clone())?);
            current_chunk.clear();
        } else {
            current_chunk.push(line);
        }
    }
    if !current_chunk.is_empty() {
        maps.push(Map::try_from(current_chunk)?);
    }

    Ok(maps)
}

fn read_seeds(lines: &[String]) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    lines[0]
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .map_err(Into::into)
}

struct Data {
    maps: Vec<Map>,
    seeds: Vec<i64>,
}

fn read_data(path: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let lines = &read_lines(path);

    let maps = read_maps(lines)?;
    let seeds = read_seeds(lines)?;

    Ok(Data { maps, seeds })
}

fn main() {
    let path = "res/data.txt";
    // let path = "res/data_light.txt";

    if let Ok(Data { seeds, maps }) = read_data(path) {
        let from_map: HashMap<String, &Map> =
            maps.iter().map(|map| (map.from.clone(), map)).collect();

        let start = "seed";
        let end = "location";

        let find_location = |seed| {
            println!("Finding location for seed: {}", seed);
            let mut current_map = from_map.get(start).unwrap();
            let mut current_value = seed;

            while !current_map.to.eq(end) {
                current_value = current_map.convert(current_value);
                current_map = from_map.get(&current_map.to).unwrap();
            }
            current_value = current_map.convert(current_value);
            println!("\tLocation found: {}", current_value);

            current_value
        };

        let mut min_location = i64::MAX;
        for seed in seeds {
            let location = find_location(seed);
            if location < min_location {
                min_location = location;
            }
        }

        println!("Minimum location: {}", min_location);
    } else {
        eprintln!("Error reading data");
    }
}
