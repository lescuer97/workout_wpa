declare global {
  // It requires an extra line to pull out the values
  type Muscle = typeof Muscle[keyof typeof Muscle];
  type WeightUnit = typeof WeightUnit[keyof typeof WeightUnit];
  type WorkoutType = typeof WorkoutType[keyof typeof WorkoutType];

  type Excersize = {
    name: string;
    weight: number;
    sets: number;
    media_url: string;
    rest: number;
    weight_unit: WeightUnit;
    reps: number;
    used_muscles: Array<Muscle>;
    workout_type: string;
  };

  type WorkoutList = {
    workouts: Array<Excersize>;
  };

  type ErrorFromCreation = {
    result: string;
    data: string;
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
