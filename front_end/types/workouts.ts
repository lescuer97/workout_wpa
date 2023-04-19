declare global {
  // It requires an extra line to pull out the values
  type Muscle = typeof Muscle[keyof typeof Muscle];
  type WeightUnit = typeof WeightUnit[keyof typeof WeightUnit];
  type WorkoutType = typeof WorkoutType[keyof typeof WorkoutType];
  type UserRole = typeof UserRole[keyof typeof UserRole];

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

  type UserRegistration = {
    email: string;
    password: string;
    password_repeat: string;
  };

  type WorkoutList = {
    workouts: Array<Excersize>;
  };
}

export const UserRole = {
  EditSelf: "EditSelf",
  EditOther: "EditOther",
  RemoveOther: "RemoveOther",
  WatchOther: "WatchOther",
  SuperAdmin: "SuperAdmin",
} as const;

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
