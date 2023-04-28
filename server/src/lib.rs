use actix_web::web::Data;
use db::register_excersice;
use error::UserError;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    FromRow, Pool, Postgres, Type,
};
use uuid::Uuid;
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
pub mod utils;

#[derive(Clone, PartialEq, Serialize, Debug, Deserialize, Type)]
#[sqlx(type_name = "muscle")]
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

impl PgHasArrayType for Muscle {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_muscle")
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "weight_unit")]
pub enum WeightUnit {
    #[default]
    KILOGRAMS,
    POUNDS,
}
impl PgHasArrayType for WeightUnit {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_weight_unit")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "workout_type")]
pub enum WorkoutType {
    CALISTHENICS,
    WEIGHTS,
    MACHINE,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Excercise {
    pub name: String,
    pub weight: i16,
    pub weight_unit: WeightUnit,
    pub sets: i16,
    pub reps: i16,
    pub rest: i16,
    pub media_url: String,
    pub used_muscles: Vec<Muscle>,
    pub workout_type: WorkoutType,
    pub id: Option<Uuid>,
}
impl Excercise {
    pub fn new(
        name: String,
        weight: i16,
        media_url: String,
        sets: i16,
        rest: i16,
        reps: i16,
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
            id: Some(Uuid::new_v4()),
        };
    }

    pub async fn register(&mut self, pool: Data<Pool<Postgres>>) -> Result<(), UserError> {
        register_excersice(self.clone(), pool).await?;

        return Ok(());
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
        let id = excercise.id;

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
                workout_type: WorkoutType::WEIGHTS,
                id: id
            }
        );
    }
    // #[test]
    // fn create_workout_list() {
    //     let ex_list: WorkoutList = WorkoutList::new(vec![
    //         Excercise::new(
    //             "Bench Press".to_string(),
    //             100,
    //             "".to_string(),
    //             3,
    //             60,
    //             8,
    //             WeightUnit::KILOGRAMS,
    //             vec![Muscle::CHEST, Muscle::FRONT_DELTS],
    //             WorkoutType::WEIGHTS,
    //         ),
    //         Excercise::new(
    //             "Pull Up".to_string(),
    //             100,
    //             "".to_string(),
    //             3,
    //             60,
    //             8,
    //             WeightUnit::KILOGRAMS,
    //             vec![Muscle::REAR_DELTS, Muscle::BACK],
    //             WorkoutType::CALISTHENICS,
    //         ),
    //         Excercise::new(
    //             "Incline Bench Press".to_string(),
    //             100,
    //             "".to_string(),
    //             3,
    //             60,
    //             8,
    //             WeightUnit::KILOGRAMS,
    //             vec![Muscle::CHEST, Muscle::FRONT_DELTS],
    //             WorkoutType::WEIGHTS,
    //         ),
    //     ]);
    //
    //     let id0 = ex_list.workouts[0].id;
    //     let id1 = ex_list.workouts[1].id;
    //     let id2 = ex_list.workouts[2].id;
    //
    //     assert_eq!(
    //         ex_list,
    //         WorkoutList {
    //             workouts: vec![
    //                 Excercise::new(
    //                     "Bench Press".to_string(),
    //                     100,
    //                     "".to_string(),
    //                     3,
    //                     60,
    //                     8,
    //                     WeightUnit::KILOGRAMS,
    //                     vec![Muscle::CHEST, Muscle::FRONT_DELTS],
    //                     WorkoutType::WEIGHTS
    //                     id :id0
    //                 ),
    //                 Excercise::new(
    //                     "Pull Up".to_string(),
    //                     100,
    //                     "".to_string(),
    //                     3,
    //                     60,
    //                     8,
    //                     WeightUnit::KILOGRAMS,
    //                     vec![Muscle::REAR_DELTS, Muscle::BACK],
    //                     WorkoutType::CALISTHENICS
    //                 ),
    //                 Excercise::new(
    //                     "Incline Bench Press".to_string(),
    //                     100,
    //                     "".to_string(),
    //                     3,
    //                     60,
    //                     8,
    //                     WeightUnit::KILOGRAMS,
    //                     vec![Muscle::CHEST, Muscle::FRONT_DELTS],
    //                     WorkoutType::WEIGHTS
    //                 ),
    //             ]
    //         }
    // )
    // }
}
