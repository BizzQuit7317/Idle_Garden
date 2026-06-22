use super::FeederSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use macroquad::rand::gen_range;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq, PartialOrd, Ord,  Copy)]
pub enum BirdFamily {
    SmallPearching,
    Starlings,
    Doves,
    Corvids,
    Raptors,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FeederDefinition {
    pub feeder_id: String,
    pub display_name: String,
    pub food_capacity: u32,
    pub bird_capacity: u32,
    pub largest_family: BirdFamily,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FoodDefinition {
    pub food_id: String,
    pub display_name: String,
    pub quantity: u32,
    pub decay_rate: f64, //how many ticks before 1 piece of food is lost without being eaten
    pub attractiveness: HashMap<BirdFamily, f64>, // <FamilyName, weight>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BirdDefinition {
    pub bird_id: String,
    pub display_name: String,
    pub family: BirdFamily,
    pub dominance: f64,
    pub base_spawn_weight: f64,
    pub max_time: f64,
    pub drop_table: Vec<(String, f64)>,
}

pub fn load_feeder_definitions() -> Vec<FeederDefinition> {
    let json = include_str!("assets/feeders.json");
    serde_json::from_str(json).expect("Failed to parse feeders.json")
}

pub fn load_food_definitions() -> Vec<FoodDefinition> {
    let json = include_str!("assets/foods.json");
    serde_json::from_str(json).expect("Failed to parse foods.json")
}

pub fn load_bird_definitions() -> Vec<BirdDefinition> {
    let json = include_str!("assets/birds.json");
    serde_json::from_str(json).expect("Failed to parse birds.json")
}

pub fn build_bird_pool(bird_definitions: &Vec<BirdDefinition>, feeder: &FeederDefinition, food: &FoodDefinition) -> Vec<(BirdDefinition, f64)> {
    bird_definitions.iter().filter(|b| b.family <= feeder.largest_family).map(|b| {
        let food_weight = food.attractiveness.get(&b.family).copied().unwrap_or(0.0);
        let weight = b.base_spawn_weight * food_weight;
        (b.clone(), weight) 
    }).collect()
}

pub fn try_spawn_birds(feeder: &mut FeederSystem) {
    feeder.spawn_ticker += 1.0;
    if feeder.spawn_ticker < 10.0 { 
        return; 
    }
    feeder.spawn_ticker = 0.0;

    if feeder.current_feeder.is_none() || feeder.current_food_amount == 0 || feeder.bird_pool.is_empty() { 
        return; 
    }

    let capacity  = feeder.current_feeder.as_ref().unwrap().bird_capacity as usize;
    let available_space = capacity.saturating_sub(feeder.current_birds.len());
    if available_space == 0 { 
        return; 
    }

    let mut incoming: Vec<(BirdDefinition, f64)> = feeder.bird_pool
        .iter()
        .filter_map(|(bird, weight)| {
            let roll: f64 = gen_range(0.0, 1.0);
            if roll < *weight {
                Some((bird.clone(), 0.0))
            } else {
                None
            }
        })
        .collect();

    let mut all_birds = feeder.current_birds.clone();
    all_birds.extend(incoming);
    resolve_dominance(&mut all_birds);
    feeder.current_birds = all_birds;
}

pub fn resolve_dominance(birds: &mut Vec<(BirdDefinition, f64)>) {
    let mut families: Vec<BirdFamily> = birds.iter().map(|(b, _)| b.family.clone()).collect();
    families.dedup();
    families.sort();

    for i in 0..families.len() {
        for j in (i+1)..families.len() {
            let high_family = &families[j];
            let low_family = &families[i];

            // skip if either family has already been cleared by a previous iteration
            if !birds.iter().any(|(b, _)| &b.family == high_family) { continue; }
            if !birds.iter().any(|(b, _)| &b.family == low_family) { continue; }

            let steps_apart = *high_family as i32 - *low_family as i32;

            if steps_apart == 1 {
                // one step apart, dominance decides
                let high_dominance: f64 = birds.iter().filter(|(b, _)| &b.family == high_family).map(|(b, _)| b.dominance).sum();
                let low_dominance: f64 = birds.iter().filter(|(b, _)| &b.family == low_family).map(|(b, _)| b.dominance).sum();

                if high_dominance > low_dominance {
                    // high family wins, remove lowest dominance birds from low family until dominance tips
                    let mut low_birds: Vec<usize> = birds.iter().enumerate()
                        .filter(|(_, (b, _))| &b.family == low_family)
                        .map(|(i, _)| i)
                        .collect();
                    low_birds.sort_by(|&a, &b| birds[a].0.dominance.partial_cmp(&birds[b].0.dominance).unwrap());

                    let mut running_low = low_dominance;
                    for idx in low_birds {
                        if running_low <= high_dominance { break; }
                        running_low -= birds[idx].0.dominance;
                        birds.remove(idx);
                    }
                } else {
                    // low family wins, high family leaves entirely
                    birds.retain(|(b, _)| &b.family != high_family);
                }
            } else {
                // two or more steps apart, low family always leaves entirely
                birds.retain(|(b, _)| &b.family != low_family);
            }
        }
    }
}

pub fn tick_birds(feeder: &mut FeederSystem) {
    let mut food_consumed = 0u32;

    feeder.current_birds.retain_mut(|(bird, ticker)| {
        *ticker += 1.0;
        let eat_chance = *ticker / bird.max_time;
        let roll: f64 = gen_range(0.0, 1.0);

        if roll < eat_chance {
            // bird eats and leaves
            food_consumed += 1;
            false
        } else if *ticker >= bird.max_time {
            // bird times out and leaves without eating
            false
        } else {
            // bird stays
            true
        }
    });

    feeder.current_food_amount = feeder.current_food_amount.saturating_sub(food_consumed);
}

pub fn tick(feeder: &mut FeederSystem, ctx: &ResourceContext) -> SubsystemOutput {
    let mut output = SubsystemOutput::empty();

    try_spawn_birds(feeder);
    tick_birds(feeder);

    // Handle pending feeder
    if let Some(def) = feeder.pending_feeder.take() {
        output.items_consumed.push((def.feeder_id.clone(), 1));
        feeder.current_feeder = Some(def);
    }

    // Handle pending food
    if let Some(def) = feeder.pending_food.take() {
        feeder.current_food_amount = def.quantity;
        output.items_consumed.push((def.food_id.clone(), 1));
        feeder.current_food = Some(def);
        //Build the bird pool here for each time food is added
        feeder.bird_pool = build_bird_pool(&feeder.bird_definitions, feeder.current_feeder.as_ref().unwrap(), feeder.current_food.as_ref().unwrap()); 
    }

    // Decay logic
    if let Some(ref food) = feeder.current_food {
        if feeder.current_food_amount > 0 {
            if feeder.decay_rate_ticker >= food.decay_rate {
                feeder.current_food_amount -= 1;
                feeder.decay_rate_ticker = 0.0;
            } else {
                feeder.decay_rate_ticker += 1.0;
            }
        }
    }

    output
}