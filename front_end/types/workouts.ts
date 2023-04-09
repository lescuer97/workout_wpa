declare global {
  type Excersize = {
    name: string;
    weight: number;
    sets: number;
    media_url: string;
    rest: number;
    weight_unit: WeightUnit;
    reps: number;
    used_muscles: Array<Muscle>;
    workout_type: WorkoutType;
  };

  type WorkoutList = {
    workouts: Array<Excersize>;
  };
}

export const WorkoutType = {
  CALISTHENICS: "CALISTHENICS",
  WEIGHTS: "WEIGHTS",
  MACHINE: "MACHINE",
} as const;

export const WeightUnit = {
  KILOGRAMS: "KILOGRAMS",
  POUNDS: "POUNDS",
} as const;

export const Muscle = {
  BICEPS: "BICEPS",
  TRICEPS: "TRICEPS",
  CHEST: "CHEST",
  BACK: "BACK",
  LEGS: "LEGS",
  LOWER_BACK: "LOWER_BACK",
  ABS: "ABS",
  LAT: "LAT",
  TRAPS: "TRAPS",
  QUADS: "QUADS",
  HAMSTRINGS: "HAMSTRINGS",
  CALVES: "CALVES",
  GLUTES: "GLUTES",
  FOREARMS: "FOREARMS",
  NECK: "NECK",
  FRONT_DELTS: "FRONT_DELTS",
  SIDE_DELTS: "SIDE_DELTS",
  REAR_DELTS: "REAR_DELTS",
} as const;
