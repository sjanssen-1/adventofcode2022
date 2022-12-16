use std::collections::HashSet;
use std::fs::read_to_string;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()>{
    let readings = read_to_string("data/day15_personal.txt")?;

    let sensor_regex = Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*)").unwrap();
    let beacon_regex = Regex::new(r"closest beacon is at x=(-?\d*), y=(-?\d*)").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();

    for reading in readings.lines() {
        let sensor_captures = sensor_regex.captures(reading).unwrap();
        let sensor_x: isize = sensor_captures.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y: isize = sensor_captures.get(2).unwrap().as_str().parse().unwrap();
        let beacons_recaptures = beacon_regex.captures(reading).unwrap();
        let beacon_x: isize = beacons_recaptures.get(1).unwrap().as_str().parse().unwrap();
        let beacon_y: isize = beacons_recaptures.get(2).unwrap().as_str().parse().unwrap();

        sensors.push(Sensor::new(sensor_x, sensor_y, beacon_x, beacon_y));
    }
    println!("{:?}", sensors);

    part1(&sensors, 2000000);
    part2(&sensors, 4000000);
    Ok(())
}

fn part2(sensors: &Vec<Sensor>, bounds: isize) {
    for y in 0..=bounds {
        let mut intervals: Vec<(isize, isize)> = Vec::new();
        for sensor in sensors {
            let offset = sensor.offset(y);
            if offset < 0 {
                continue;
            }
            let low_x = sensor.sx - offset as isize;
            let high_x = sensor.sx + offset as isize;
            intervals.push((low_x, high_x));
        }

        intervals.sort();

        let mut non_overlapping_intervals: Vec<(isize, isize)> = Vec::new();

        for (low_x, high_x) in intervals {
            if non_overlapping_intervals.is_empty() {
                non_overlapping_intervals.push((low_x, high_x));
                continue;
            }

            let (_, noi_high) = non_overlapping_intervals.last().unwrap();
            if low_x > noi_high + 1 {
                non_overlapping_intervals.push((low_x, high_x));
                continue;
            }

            non_overlapping_intervals.last_mut().unwrap().1 = *noi_high.max(&high_x);
        }

        let mut x: isize = 0;
        for (low, high) in non_overlapping_intervals {
            if x < low {
                println!("part 2: {}", x * 4000000 + y);
                return;
            }
            x = x.max(high + 1);
            if x > bounds {
                break;
            }
        }
    }
}

fn part1(sensors: &Vec<Sensor>, y: isize) {
    let mut known: HashSet<isize> = HashSet::new();
    let mut intervals: Vec<(isize, isize)> = Vec::new();
    for sensor in sensors {
        let offset = sensor.offset(y);
        if offset < 0 {
            continue;
        }
        let low_x = sensor.sx - offset as isize;
        let high_x = sensor.sx + offset as isize;
        intervals.push((low_x, high_x));

        if sensor.by == y {
            known.insert(sensor.bx);
        }
    }

    intervals.sort();

    let mut non_overlapping_intervals: Vec<(isize, isize)> = Vec::new();

    for (low_x, high_x) in intervals {
        if non_overlapping_intervals.is_empty() {
            non_overlapping_intervals.push((low_x, high_x));
            continue;
        }

        let (_, noi_high) = non_overlapping_intervals.last().unwrap();
        if low_x > noi_high + 1 {
            non_overlapping_intervals.push((low_x, high_x));
            continue;
        }

        non_overlapping_intervals.last_mut().unwrap().1 = *noi_high.max(&high_x);
    }

    let mut cannot: HashSet<isize> = HashSet::new();

    for (low, high) in non_overlapping_intervals {
        for x in low..=high {
            cannot.insert(x);
        }
    }

    println!("part 1: {}", cannot.len() - known.len());
}

#[derive(Debug)]
struct Sensor {
    sx: isize,
    sy: isize,
    bx: isize,
    by: isize,
    distance: usize,
}
impl Sensor {
    fn new(sx: isize, sy: isize, bx: isize, by: isize) -> Self {
        Self{
            sx,
            sy,
            bx,
            by,
            distance: sx.abs_diff(bx) + sy.abs_diff(by),
        }
    }

    fn offset(&self, y: isize) -> isize {
        self.distance as isize - self.sy.abs_diff(y) as isize
    }
}