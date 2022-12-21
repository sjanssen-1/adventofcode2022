use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;
use anyhow::{Result, Error};

fn main() -> Result<()>{
    let mut blueprints: Vec<Blueprint> = Vec::new();
    let input = read_to_string("data/day19.txt")?;
    for i in input.lines() {
        blueprints.push(i.parse().unwrap());
    }
    println!("{:?}", blueprints);

    // part1(&blueprints);
    part2(&blueprints);

    Ok(())
}

fn part1(blueprints: &Vec<Blueprint>) {
    let mut sum = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        let mut cache: HashMap<State, u32> = HashMap::new();
        let result = dfs(State{
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            ore_bots_in_prod: 0,
            clay_bots_in_prod: 0,
            obsidian_bots_in_prod: 0,
            geode_bots_in_prod: 0,
            minutes: 24,
        }, blueprint, &mut cache);
        println!("result for blueprint {}, was {}", idx+1, result);
        sum += (idx+1) as u32 * result;
    }
    println!("Sum is (part 1): {}", sum);
}

fn part2(blueprints: &Vec<Blueprint>) {
    let mut product = 1;
    for blueprint in &blueprints[..=1] {
        let mut cache: HashMap<State, u32> = HashMap::new();
        let result = dfs(State{
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            ore_bots_in_prod: 0,
            clay_bots_in_prod: 0,
            obsidian_bots_in_prod: 0,
            geode_bots_in_prod: 0,
            minutes: 32,
        }, blueprint, &mut cache);
        println!("result for blueprint was {}", result);
        product *= result;
    }
    println!("Product is (part 2): {}", product);
}

// return the best amount of geodes cracked
fn dfs(state: State, blueprint: &Blueprint, cache: &mut HashMap<State, u32>) -> u32 {
    if state.minutes == 0 {
        return state.geode;
    }
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }

    let can_create_geode_bot = state.ore >= blueprint.geode_cost.0 && state.obsidian >=blueprint.geode_cost.1;
    let can_create_obsidian_bot = state.ore >= blueprint.obsidian_cost.0 && state.clay >= blueprint.obsidian_cost.1 && state.obsidian_bots < blueprint.obsidian_rate;
    let can_create_clay_bot = state.ore >= blueprint.clay_cost && state.clay_bots < blueprint.clay_rate;
    let can_create_ore_bot = state.ore >= blueprint.ore_cost && state.ore_bots < blueprint.ore_rate;

    // convert producing bots to bots first and then mine the resources.. just keep the can create check first because the resources only get added at the end of the minute

    let new_state = State{
        ore: state.ore + state.ore_bots + state.ore_bots_in_prod,
        clay: state.clay + state.clay_bots + state.clay_bots_in_prod,
        obsidian: state.obsidian + state.obsidian_bots + state.obsidian_bots_in_prod,
        geode: state.geode + state.geode_bots + state.geode_bots_in_prod,
        ore_bots: state.ore_bots + state.ore_bots_in_prod,
        clay_bots: state.clay_bots + state.clay_bots_in_prod,
        obsidian_bots: state.obsidian_bots + state.obsidian_bots_in_prod,
        geode_bots: state.geode_bots + state.geode_bots_in_prod,
        ore_bots_in_prod: 0,
        clay_bots_in_prod: 0,
        obsidian_bots_in_prod: 0,
        geode_bots_in_prod: 0,
        minutes: state.minutes - 1,
    };

    let mut best = new_state.geode;

    if can_create_geode_bot {
        let mut new_state = new_state.clone();
        new_state.geode_bots_in_prod = 1;
        new_state.ore -= blueprint.geode_cost.0;
        new_state.obsidian -= blueprint.geode_cost.1;
        let path_result = dfs(new_state.clone(), blueprint, cache);
        cache.insert(new_state.clone(), path_result);
        best = best.max(path_result);
    }

    if can_create_obsidian_bot {
        let mut new_state = new_state.clone();
        new_state.obsidian_bots_in_prod = 1;
        new_state.ore -= blueprint.obsidian_cost.0;
        new_state.clay -= blueprint.obsidian_cost.1;
        let path_result = dfs(new_state.clone(), blueprint, cache);
        cache.insert(new_state.clone(), path_result);
        best = best.max(path_result);
    }

    if can_create_clay_bot {
        let mut new_state = new_state.clone();
        new_state.clay_bots_in_prod = 1;
        new_state.ore -= blueprint.clay_cost;
        let path_result = dfs(new_state.clone(), blueprint, cache);
        cache.insert(new_state.clone(), path_result);
        best = best.max(path_result);
    }

    if can_create_ore_bot {
        let mut new_state = new_state.clone();
        new_state.ore_bots_in_prod = 1;
        new_state.ore -= blueprint.ore_cost;
        let path_result = dfs(new_state.clone(), blueprint, cache);
        cache.insert(new_state.clone(), path_result);
        best = best.max(path_result);
    }

    // do nothing path
    let new_state = new_state.clone();
    let path_result = dfs(new_state.clone(), blueprint, cache);
    cache.insert(new_state.clone(), path_result);
    best = best.max(path_result);

    best
}

// option 1: do nothing
// option 2: create ore bot
// option 3: create clay bot
// option 4: create obsidian bot
// option 5: create geode bot

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_bots: u32,
    clay_bots: u32,
    obsidian_bots: u32,
    geode_bots: u32,
    ore_bots_in_prod: u32,
    clay_bots_in_prod: u32,
    obsidian_bots_in_prod: u32,
    geode_bots_in_prod: u32,
    minutes: usize,
}

#[derive(Debug)]
struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: (u32, u32),
    geode_cost: (u32, u32),
    ore_rate: u32,
    clay_rate: u32,
    obsidian_rate: u32,
}
impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        println!("{:?}", parts);
        let mut max_ore: u32 = parts[6].parse().unwrap();
        max_ore = max_ore.max(parts[12].parse().unwrap());
        max_ore = max_ore.max(parts[18].parse().unwrap());
        max_ore = max_ore.max(parts[27].parse().unwrap());
        Ok(Blueprint{
            ore_cost: parts[6].parse().unwrap(),
            clay_cost: parts[12].parse().unwrap(),
            obsidian_cost: (parts[18].parse().unwrap(), parts[21].parse().unwrap()),
            geode_cost: (parts[27].parse().unwrap(), parts[30].parse().unwrap()),
            ore_rate: max_ore,
            clay_rate: parts[21].parse().unwrap(),
            obsidian_rate: parts[30].parse().unwrap(),
        })
    }
}