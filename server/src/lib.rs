use serde::{Deserialize, Serialize, Deserializer};

// use workouts::workouts_server::{Workouts, WorkoutsServer};
// use workouts::WeightType;

// pub mod workouts {
//     include!(concat!(env!("OUT_DIR"), "/workouts.rs"));
// }

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Muscle {
    BICEPS,
    TRICEPS,
    CHEST,
    BACK,
    LEGS,
    LOWER_BACK,
    ABS,
    LAT,
    TRAPS,
    QUADS,
    HAMSTRINGS,
    CALVES,
    GLUTES,
    FOREARMS,
    NECK,
    FRONT_DELTS,
    SIDE_DELTS,
    REAR_DELTS,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeightUnit {
    KILOGRAMS,
    POUNDS,
}
impl Default for WeightUnit {
    fn default() -> Self {
        WeightUnit::KILOGRAMS
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Excercise {
    pub name: String,
    pub weight: i32,
    pub weight_unit: WeightUnit,
    pub sets: i32,
    pub reps: i32,
    pub rest: i32,
    pub media_url: String,
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkoutList {
    pub workouts: Vec<Excercise>,
}
//
// impl Excercise {
//     pub fn new(
//         name: String,
//         weight: i32,
//         media_url: String,
//         sets: i32,
//         rest: i32,
//         reps: i32,
//         weight_type: u8,
//     ) -> Excercise {
//         return Excercise {
//             name,
//             weight,
//             media_url,
//             reps,
//             sets,
//             rest,
//             weight_unit: weight_type,
//         };
//     }
// }
