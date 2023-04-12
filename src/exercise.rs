#[derive(Clone, Copy)]
pub enum Unit {
    Reps,
    Sec,
    Max,
}

#[derive(Clone)]
pub struct Exercise {
    pub name: String,
    pub duration: u16,
    pub unit: Unit,
}

impl Exercise {
    pub fn display(&self) {
        let _unit = match self.unit {
            Unit::Reps => String::from("reps"),
            Unit::Sec => String::from("sec"),
            Unit::Max => String::from("max"),
        };

        println!("{}", self.name);
        print!("{}", self.duration);
        println!("{}", _unit);
        println!("=======================")
    }
}

#[derive(Clone)]
pub struct Training {
    pub name: String,
    pub exercises: Vec<Exercise>,
    pub rest_between_exercises: usize, // in seconds
}

impl Training {
    pub fn display(&self) {
        println!("name: {}", self.name);
        println!("exercises: ");

        for i in 0..self.exercises.len() {
            println!("{}", i);
            Exercise::display(&self.exercises[i]);
        }
    }
}
