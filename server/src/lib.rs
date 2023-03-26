// use workouts::workouts_server::{Workouts, WorkoutsServer};

// use workouts::WeightType;

pub mod workouts {
    include!(concat!(env!("OUT_DIR"), "/workouts.rs"));
}

impl workouts::Excercise {
    pub fn new(
        name: String,
        weight: i32,
        media_url: String,
        sets: i32,
        rest: i32,
        weight_type: workouts::WeightType,
    ) -> workouts::Excercise {
        workouts::Excercise {
            name,
            weight,
            media_url,
            sets,
            rest,
            weight_unit: weight_type.into(),
        }
    }
}
