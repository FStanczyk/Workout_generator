use crate::exercise::{Exercise, Training, Unit};
use rand::Rng;
use std::error::Error;

#[derive(Clone, Copy)]
pub enum GOAL {
    Burn,
    Bulk,
    Progress,
}

pub fn generate_training_unit(
    goal: GOAL,
    units_per_week: usize,
    diff: u8,
    exercises: &[u8],
) -> Result<Training, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data.csv")?;

    let mut workout_list: Vec<Exercise> = vec![];
    let goal_multiplier = goal_multiplier(goal);

    let duration_data = match goal_multiplier.1 {
        Unit::Sec => 15,
        Unit::Reps => 16,
        _ => 16,
    };

    for result in rdr.records() {
        let record = result?;
        let id: u8 = record[0].parse()?;
        if exercises.contains(&id) {
            let multiplier: f32 = goal_multiplier.0
                * week_units_multiplier(units_per_week)
                * difficulty_multiplier(diff, record[2].parse()?);

            let dur: usize = record[duration_data].parse()?;

            workout_list.push(Exercise {
                name: record[1].parse()?,
                duration: (dur as f32 * multiplier) as u16,
                unit: goal_multiplier.1,
            })
        }
    }

    let number_of_exercises = calc_ex_per_set(goal, units_per_week, diff);

    while workout_list.len() != number_of_exercises {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..workout_list.len());
        workout_list.remove(i);
    }

    Ok(Training {
        name: String::from("Training unit"),
        exercises: workout_list,
        rest_between_exercises: calc_rest(goal, diff as usize),
    })
}

fn goal_multiplier(goal: GOAL) -> (f32, Unit) {
    match goal {
        GOAL::Burn => return (1.1, Unit::Sec),
        GOAL::Bulk => return (1.0, Unit::Reps),
        GOAL::Progress => return (0.9, Unit::Reps),
    }
}

fn week_units_multiplier(units_per_week: usize) -> f32 {
    let mux: i8 = 4 - units_per_week as i8;

    if mux == 0 {
        return 1.0;
    } else if mux < 0 {
        return 1.0 + (mux as f32 / 10.0);
    } else {
        return 1.0 - (mux as f32 / 10.0);
    }
}

fn difficulty_multiplier(diff: u8, ex_diff: u8) -> f32 {
    let mux = ex_diff as i8 - diff as i8;

    if mux == 0 {
        return 1.0;
    } else if mux < 0 {
        return 1.0 + (-mux as f32) / 10.0;
    } else {
        return 1.0 - (mux as f32) / 10.0;
    }
}

fn calc_ex_per_set(goal: GOAL, units_per_week: usize, diff: u8) -> usize {
    let mut result: usize = match goal {
        GOAL::Burn => 9,
        GOAL::Bulk => 6,
        GOAL::Progress => 5,
    };

    match units_per_week {
        1 => result += 7,
        2 => result += 5,
        3 => result += 2,
        4 => result += 0,
        5 => result += 0,
        6 => result -= 1,
        7 => result -= 1,
        _ => result -= 1,
    };

    if diff > 8 {
        result -= 1;
    } else if diff > 6 {
        result += 0;
    } else if diff > 4 {
        result += 1;
    } else {
        result += 2
    }

    return result;
}

// In seconds
//(between exercises, NOT SETS!)
fn calc_rest(goal: GOAL, diff: usize) -> usize {
    let mut rest: usize = match goal {
        GOAL::Burn => 25,
        GOAL::Bulk => 45,
        GOAL::Progress => 60,
    };

    if diff > 8 {
        rest -= 10;
    } else if diff > 6 {
        rest -= 5;
    } else if diff > 4 {
        rest += 0;
    } else {
        rest += 5
    }

    return rest;
}


pub fn generate_week_plan(goal: GOAL, units_per_week: usize, diff: u8, exercises: &[u8]) ->  Vec<Training> {
    let mut units: Vec<Training> = vec![];
    let mut rng = rand::thread_rng();


    for _i in 0..units_per_week {
        let pls = rng.gen_range(0..1);
        let min = rng.gen_range(0..1);
        let unit = generate_training_unit(goal, units_per_week+pls-min, diff, exercises).expect("Couldn't generate unit"); 
        units.push(unit);
    }

    units
}