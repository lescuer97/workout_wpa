import { Muscle, WeightUnit, WorkoutType } from "@/types/workouts.ts";

export default function Button_send() {
  async function ask_for_workout() {
    const excersise: Excersize = {
      name: "Benchpress",
      sets: 3,
      reps: 10,
      weight: 100,
      weight_unit: WeightUnit.KILOGRAMS,
      rest: 15,
      media_url: "",
      used_muscles: [Muscle.CHEST, Muscle.TRICEPS],
      workout_type: WorkoutType.WEIGHTS,
    };

    const json_excersise = JSON.stringify(excersise);

    const res = await fetch("http://127.0.0.1:8080/workout", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },

      body: json_excersise,
    });

    await res.json().then((data: Excersize) => {
      console.log({ data });
    });
  }

  return (
    <div>
      <button onClick={() => ask_for_workout()}>Check workout</button>
    </div>
  );
}
