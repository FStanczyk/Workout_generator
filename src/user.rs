#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

enum MUSCLE {
    Bicep, 
    Tricep,
    Arm,
    ABS,
    Chest,
    Back,
    Calves,
    Legs
}

use crate::generator;
fn make_me_a_plan(
    bicep: u8,
    tricep: u8,
    arm: u8,
    abs: u8,
    chest: u8,
    back: u8,
    calves: u8,
    legs: u8,

    diff: u8,
    units_per_week: u8,
    goal: generator::GOAL
){
    
}