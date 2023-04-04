import { WeightUnit } from "@/types/workouts.ts";

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
    }

    const json_excersise = JSON.stringify(excersise);
    
    console.log({excersise});
    console.log({json_excersise});

    const res = await fetch("http://127.0.0.1:8080/workout", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      
      body: json_excersise
    });


    await res.json().then((data: Excersize) => {
      console.log( {data});
    })

  }

  return (
    <div>
      <button onClick={() => ask_for_workout()}>Check workout</button>
    </div>
  );
}
