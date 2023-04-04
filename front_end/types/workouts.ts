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
  };

  type WorkoutList = {
    workouts: Array<Excersize>;
  };
}
export enum WeightUnit {
  KILOGRAMS = "KILOGRAMS",
  POUNDS = "POUNDS",
}

export enum Muscle {
  BICEPS = "BICEPS",
  TRICEPS = "TRICEPS",
  CHEST = "CHEST",
  BACK = "BACK",
  LEGS = "LEGS",
  LOWER_BACK = "LOWER_BACK",
  ABS = "ABS",
  LAT = "LAT",
  TRAPS = "TRAPS",
  QUADS = "QUADS",
  HAMSTRINGS = "HAMSTRINGS",
  CALVES = "CALVES",
  GLUTES = "GLUTES",
  FOREARMS = "FOREARMS",
  NECK = "NECK",
  FRONT_DELTS = "FRONT_DELTS",
  SIDE_DELTS = "SIDE_DELTS",
  REAR_DELTS = "REAR_DELTS",
}
