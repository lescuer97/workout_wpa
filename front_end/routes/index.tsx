import { Head } from "$fresh/runtime.ts";
import { Handlers, PageProps } from "$fresh/server.ts";
import { Muscle, WeightUnit, WorkoutType } from "@/types/workouts.ts";

import Input from "@/islands/Input.tsx";
import Select from "@/islands/Select.tsx";

interface Data {
  results: Excersize;
  query: string;
}
export const handler: Handlers<Data> = {
  async POST(req, ctx) {
    const data: FormData = await req.formData<string>();

    const queryString = new URLSearchParams(data).toString();

    const res = await fetch(`http://127.0.0.1:8080/workout?${queryString}`, {
      method: "POST",
    });

    const ex = await res.json() as Excersize;

    return ctx.render({ results: ex, query: "hello world" });
  },
};

export default function Home({ data }: PageProps<Data>) {
  if (data) {
    return (
      <>
        <Head>
          <title>Fresh App</title>
        </Head>
        <div>
          hello world
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
