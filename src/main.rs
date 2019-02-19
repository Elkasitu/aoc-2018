use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("Checking for input.txt...");
    let mut input = String::new();
    let mut f = File::open("src/input.txt").unwrap();
    f.read_to_string(&mut input);
    let coords: Vec<&str> = input.trim().split('\n').collect();
    println!("Part 1 solution: {}", part_1(coords.clone()));
    println!("Part 2 solution: {}", part_2(coords));
}

struct Plot {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Plot {
    fn get_plot_coords(&self) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        for i in 0..self.width {
            for j in 0..self.height {
                result.push((i + self.x, j + self.y));
            }
        }
        return result;
    }
}

fn parse_coord(coord: &str, id: i32) -> Plot {
    let idrest: Vec<&str> = coord.split('@').collect();
    let startdim: Vec<&str> = idrest[1].trim().split(':').collect();
    let xy: Vec<&str> = startdim[0].trim().split(',').collect();
    let x: i32 = xy[0].parse().unwrap();
    let y: i32 = xy[1].parse().unwrap();
    let dim: Vec<&str> = startdim[1].trim().split('x').collect();
    let w: i32 = dim[0].parse().unwrap();
    let h: i32 = dim[1].parse().unwrap();
    return Plot { id: id, x: x, y: y, width: w, height: h };
}

fn parse_coords(coords: Vec<&str>) -> Vec<Plot> {
    let mut result: Vec<Plot> = Vec::new();
    for (i, coord) in coords.iter().enumerate() {
        result.push(parse_coord(coord, (i as i32) + 1));
    }
    return result;
}

fn get_overlaps(plots: Vec<Plot>) -> HashMap<(i32, i32), i32> {
    let mut overlaps: HashMap<(i32, i32), i32> = HashMap::new();
    for plot in plots.iter() {
        let plot_spaces: Vec<(i32, i32)> = plot.get_plot_coords();
        for space in plot_spaces {
            *overlaps.entry(space).or_insert(0) += 1;
        }
    }
    return overlaps;
}

fn get_overlapping_sqm(overlaps: HashMap<(i32, i32), i32>, n: i32) -> i32 {
    let mut i = 0;
    for v in overlaps.values() {
        if v >= &n {
            i += 1;
        }
    }
    return i;
}

fn part_1(coords: Vec<&str>) -> i32 {
    let overlaps = get_overlaps(parse_coords(coords));
    return get_overlapping_sqm(overlaps, 2);
}

fn get_perfect_plot(plots: Vec<Plot>, overlaps: HashMap<(i32, i32), i32>) -> i32 {
    for plot in plots {
        let points = plot.get_plot_coords();
        let mut found = true;
        for point in points {
            if *overlaps.get(&point).unwrap() > 1 {
                found = false;
                break;
            }
        }
        if found {
            return plot.id;
        }
    }
    return -1;
}

fn part_2(coords: Vec<&str>) -> i32 {
    let plots: Vec<Plot> = parse_coords(coords.clone());
    let plots2: Vec<Plot> = parse_coords(coords);
    let overlaps = get_overlaps(plots);
    return get_perfect_plot(plots2, overlaps);
}
