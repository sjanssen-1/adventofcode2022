use anyhow::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<()> {
    let scan_output = read_to_string("data/day16_personal.txt")?;
    let valves: Vec<Valve> = scan_output
        .lines()
        .map(|s| s.parse::<Valve>().unwrap())
        .collect();

    let mut cache: HashMap<CacheKey, usize> = HashMap::new();
    println!("pressure (part 1): {}", dfs("AA", 30, Vec::new(),0, &valves, &mut cache));

    Ok(())
}

fn dfs(
    valve: &str,
    minutes: usize,
    opened: Vec<String>,
    pressure: usize,
    valves: &Vec<Valve>,
    cache: &mut HashMap<CacheKey, usize>,
) -> usize {
    // println!("minutes left: {}", minutes);

    if  minutes == 0 {
        return pressure;
    }

    let cache_key = CacheKey::new(valve, minutes, opened.clone());
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let current_valve = valves.iter().find(|v| v.name == valve).unwrap();

    let mut best_choice = 0;
    if current_valve.flow_rate > 0 && !opened.contains(&current_valve.name) {
        let mut new_opened = opened.clone();
        let new_pressure = pressure + ((minutes-1)*current_valve.flow_rate);
        new_opened.push(current_valve.name.to_string());
        // println!("opening valve {} ; pressure becomes {}", valve, new_pressure);
        let result = dfs(valve, minutes-1, new_opened.clone(), new_pressure, valves, cache);
        cache.insert(CacheKey::new(valve, minutes, new_opened.clone()), result);
        best_choice = result;
    }
    let valve_options = &current_valve.valves;
    for valve_option in valve_options {
        // println!("moving to valve {} ; pressure remains {}", valve_option, pressure);
        let result = dfs(valve_option, minutes-1, opened.clone(), pressure, valves, cache);
        cache.insert(CacheKey::new(valve_option, minutes-1, opened.clone()), result);
        if best_choice < result {
            best_choice = result;
        }
    }
    return best_choice;
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    valve: String,
    minutes: usize,
    opened: Vec<String>,
}
impl CacheKey {
    fn new(valve: &str, minutes: usize, opened: Vec<String>) -> Self {
        CacheKey{
            valve: valve.to_string(),
            minutes,
            opened,
        }
    }
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    valves: Vec<String>,
}
impl Valve {
    fn new(name: &str, flow_rate: usize, valves: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            flow_rate,
            valves,
        }
    }
}
impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+|[A-Z]{2})").unwrap();
        let mut valve_name = String::new();
        let mut flow_rate = 0;
        let mut valves_to = Vec::new();
        for (idx, re_match) in re.find_iter(s).enumerate() {
            match idx {
                0 => valve_name = s[re_match.start()..re_match.end()].to_string(),
                1 => {
                    flow_rate = s[re_match.start()..re_match.end()]
                        .to_string()
                        .parse::<usize>()
                        .unwrap()
                }
                _ => valves_to.push(s[re_match.start()..re_match.end()].to_string()),
            }
        }
        Ok(Valve::new(&valve_name, flow_rate, valves_to))
    }
}