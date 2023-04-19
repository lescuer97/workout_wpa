use serde::{Deserialize, Serialize};
#[allow(non_camel_case_types)]
// use workouts::workouts_server::{Workouts, WorkoutsServer};
// use workouts::WeightType;

// pub mod workouts {
//     include!(concat!(env!("OUT_DIR"), "/workouts.rs"));
// }
pub mod auth;
pub mod db;
pub mod error;
pub mod server_messages;

#[derive(Clone, PartialEq, Serialize, Debug, Deserialize)]
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeightUnit {
    #[default]
    KILOGRAMS,
    POUNDS,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkoutType {
    CALISTHENICS,
    WEIGHTS,
    MACHINE,
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
    pub used_muscles: Vec<Muscle>,
    pub workout_type: WorkoutType,
}
impl Excercise {
    pub fn new(
        name: String,
        weight: i32,
        media_url: String,
        sets: i32,
        rest: i32,
        reps: i32,
        weight_unit: WeightUnit,
        used_muscles: Vec<Muscle>,
        workout_type: WorkoutType,
    ) -> Excercise {
        return Excercise {
            name,
            weight,
            media_url,
            reps,
            sets,
            rest,
            weight_unit,
            used_muscles,
            workout_type,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkoutList {
    pub workouts: Vec<Excercise>,
}

impl WorkoutList {
    pub fn new(workouts: Vec<Excercise>) -> WorkoutList {
        return WorkoutList { workouts };
    }

    pub fn pop(&mut self) -> Option<Excercise> {
        self.workouts.pop()
    }

    pub fn push(&mut self, workout: Excercise) {
        self.workouts.push(workout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // check if the excercise is created correctly
    #[test]
    fn new_excersice() {
        let excercise: Excercise = Excercise::new(
            "Bench Press".to_string(),
            100,
            "".to_string(),
            3,
            60,
            8,
            WeightUnit::KILOGRAMS,
            vec![Muscle::CHEST, Muscle::FRONT_DELTS],
            WorkoutType::WEIGHTS,
        );

        assert_eq!(
            excercise,
            Excercise {
                name: "Bench Press".to_string(),
                weight: 100,
                weight_unit: WeightUnit::KILOGRAMS,
                sets: 3,
                reps: 8,
                rest: 60,
                media_url: "".to_string(),
                used_muscles: vec![Muscle::CHEST, Muscle::FRONT_DELTS],
                workout_type: WorkoutType::WEIGHTS
            }
        );
    }
    #[test]
    fn create_workout_list() {
        let ex_list: WorkoutList = WorkoutList::new(vec![
            Excercise::new(
                "Bench Press".to_string(),
                100,
                "".to_string(),
                3,
                60,
                8,
                WeightUnit::KILOGRAMS,
                vec![Muscle::CHEST, Muscle::FRONT_DELTS],
                WorkoutType::WEIGHTS,
            ),
            Excercise::new(
                "Pull Up".to_string(),
                100,
                "".to_string(),
                3,
                60,
                8,
                WeightUnit::KILOGRAMS,
                vec![Muscle::REAR_DELTS, Muscle::BACK],
                WorkoutType::CALISTHENICS,
            ),
            Excercise::new(
                "Incline Bench Press".to_string(),
                100,
                "".to_string(),
                3,
                60,
                8,
                WeightUnit::KILOGRAMS,
                vec![Muscle::CHEST, Muscle::FRONT_DELTS],
                WorkoutType::WEIGHTS,
            ),
        ]);

        assert_eq!(
            ex_list,
            WorkoutList {
                workouts: vec![
                    Excercise::new(
                        "Bench Press".to_string(),
                        100,
                        "".to_string(),
                        3,
                        60,
                        8,
                        WeightUnit::KILOGRAMS,
                        vec![Muscle::CHEST, Muscle::FRONT_DELTS],
                        WorkoutType::WEIGHTS
                    ),
                    Excercise::new(
                        "Pull Up".to_string(),
                        100,
                        "".to_string(),
                        3,
                        60,
                        8,
                        WeightUnit::KILOGRAMS,
                        vec![Muscle::REAR_DELTS, Muscle::BACK],
                        WorkoutType::CALISTHENICS
                    ),
                    Excercise::new(
                        "Incline Bench Press".to_string(),
                        100,
                        "".to_string(),
                        3,
                        60,
                        8,
                        WeightUnit::KILOGRAMS,
                        vec![Muscle::CHEST, Muscle::FRONT_DELTS],
                        WorkoutType::WEIGHTS
                    ),
                ]
            }
        )
    }
}
