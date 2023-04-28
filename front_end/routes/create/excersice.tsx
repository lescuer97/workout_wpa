import { Head } from "$fresh/runtime.ts";
import { Handlers, PageProps } from "$fresh/server.ts";
import { Muscle, WeightUnit, WorkoutType } from "@/types/workouts.ts";

import Input from "@/islands/Input.tsx";
import Select from "@/islands/Select.tsx";

export const handler: Handlers<ErrorFromCreation> = {
  async POST(req, ctx) {
    const data: FormData = await req.formData<string>();

    const queryString = new URLSearchParams(data).toString();

    const res = await fetch(`http://127.0.0.1:8080/workout?${queryString}`, {
      method: "POST",
    });
    if (!res.ok) {
      return ctx.render({
        result: "error",
        data: "There was an error while creating the excersice",
      });
    }

    const ex = await res.json() as Excersize;

    return ctx.render({
      result: "success",
      data: "Excersize created with success",
    });
  },
};

export default function Home({ data }: PageProps<ErrorFromCreation>) {
  if (data) {
    return (
      <>
        <div>
          {data.data}
        </div>
      </>
    );
  } else {
    return (
      <>
        <Head>
          <title>Fresh App</title>
        </Head>
        <div class="flex justify-center flex-row justify-items-center w-full">
          <form method="POST" class="flex flex-col">
            <Input
              value="press"
              type="text"
              placeholder="Name"
              name="name"
            />
            <Input
              value={2}
              type="number"
              placeholder="Sets"
              name="sets"
            />
            <Input value={5} type="number" placeholder="Reps" name="reps" />
            <Input value={5} type="number" placeholder="Weight" name="weight" />
            <Input value={5} type="number" placeholder="Rest" name="rest" />
            <Input
              value=""
              type="text"
              placeholder="Media URL"
              name="media_url"
            />
            <Select
              placeholder="Workout Type"
              name="workout_type"
              data-options={WorkoutType}
            />
            <Select
              placeholder="Weight Unit"
              name="weight_unit"
              data-options={WeightUnit}
            />
            <Select
              placeholder="Used Muscle"
              multiple
              name="used_muscles[]"
              data-options={Muscle}
            />

            <button type="submit">Check workout</button>
          </form>
        </div>
      </>
    );
  }
}
