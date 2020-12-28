mod test_solutions;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use log::{debug, info};
use std::collections::{HashSet};

fn parse_direction(direction: &str) -> i32 {
    return match direction {
        "e" => 0,
        "ne" => 1,
        "nw" => 2,
        "w" => 3,
        "sw" => 4,
        "se" => 5,
        _ => - 1
    }
}

fn get_input_data(filename: &str) -> Vec<Vec<i32>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let direction_pattern = Regex::new(r"se|e|sw|nw|w|ne").unwrap();

    let mut tiles: Vec<Vec<i32>> = vec![];

    for line in f.lines().map(|s|s.unwrap()) {
        let match_iter = direction_pattern.find_iter(&line);
        let mut current_line: Vec<i32> = vec![];
        for direction in match_iter {
            current_line.push(parse_direction(direction.as_str()));
        }
        tiles.push(current_line);
    }
    return tiles;
}

fn get_coordination_from_path(path: &Vec<i32>) -> (i32, i32) {
    let mut current_coordination = (0,0);
    for dir in path {
        match dir {
            0 => current_coordination = (current_coordination.0, current_coordination.1 + 2),
            1 => current_coordination = (current_coordination.0 - 1, current_coordination.1 + 1),
            2 => current_coordination = (current_coordination.0 - 1, current_coordination.1 - 1),
            3 => current_coordination = (current_coordination.0, current_coordination.1 - 2),
            4 => current_coordination = (current_coordination.0 + 1, current_coordination.1 - 1),
            5 => current_coordination = (current_coordination.0 + 1, current_coordination.1 + 1),
            _ => {}
        }
    }
    return current_coordination;
}

fn get_flipped_tiles(tiles: Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let tile_coords: Vec<(i32, i32)> = tiles.iter().
        map(|p| get_coordination_from_path(p)).collect();
    let mut flipped_tiles: HashSet<(i32, i32)> = HashSet::new();
    for coord in &tile_coords {
        if flipped_tiles.contains(coord) {
            flipped_tiles.remove(coord);
        } else {
            flipped_tiles.insert(*coord);
        }
    }
    return flipped_tiles;
}

fn solution_part_1(filename: &str) -> usize {
    let tiles = get_input_data(filename);
    let flipped_tiles: HashSet<(i32, i32)> = get_flipped_tiles(tiles);
    return flipped_tiles.len();
}

fn get_number_of_flipped_neighbors(x: i32, y: i32, flipped_tiles: &HashSet<(i32, i32)>) -> i32 {
    let mut count = 0;
    for neigh in vec![(0, 2), (-1, 1), (-1, -1), (0, -2), (1, -1), (1, 1)] {
        if flipped_tiles.contains(&(x + neigh.0, y + neigh.1)) {
            count += 1;
        }
    }
    return count;
}

fn cycle(flipped_tiles: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_flipped_tiles: HashSet<(i32, i32)> = HashSet::new();
    for flipped in &flipped_tiles {
        let neigh_count = get_number_of_flipped_neighbors(flipped.0, flipped.1,
                                                          &flipped_tiles);
        if neigh_count == 1 || neigh_count == 2 {
            new_flipped_tiles.insert(*flipped);
        }
        for neigh in vec![(0, 2), (-1, 1), (-1, -1), (0, -2), (1, -1), (1, 1)] {
            if flipped_tiles.contains(&(flipped.0 + neigh.0, flipped.1 + neigh.1)) {
                continue;
            }
            let neigh_count = get_number_of_flipped_neighbors(flipped.0 + neigh.0,
                                                              flipped.1 + neigh.1,
                                                              &flipped_tiles);
            if neigh_count == 2 {
                new_flipped_tiles.insert((flipped.0 + neigh.0, flipped.1 + neigh.1));
            }
        }
    }
    return new_flipped_tiles;
}

fn solution_part_2(filename: &str) -> usize {
    let tiles = get_input_data(filename);
    let mut flipped_tiles: HashSet<(i32, i32)> = get_flipped_tiles(tiles);
    debug!("{:?}", &flipped_tiles.len());
    for _i in 0..100 {
        flipped_tiles = cycle(flipped_tiles);
        debug!("{:?}", &flipped_tiles.len());
    }
    return flipped_tiles.len();
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
