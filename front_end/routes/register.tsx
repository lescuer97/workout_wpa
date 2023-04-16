import { Head } from "$fresh/runtime.ts";
import { Handlers, PageProps } from "$fresh/server.ts";
import { Muscle, WeightUnit, WorkoutType } from "@/types/workouts.ts";

import Input from "@/islands/Input.tsx";
import Select from "@/islands/Select.tsx";

interface Data {
  results: UserRegistration;
  query: string;
}

function randomNumber(min: number, max: number) {
  return Math.floor(Math.random() * (max - min) + min);
}

export const handler: Handlers<Data> = {
  async POST(req: Request, ctx) {
    const data: FormData = await req.formData();

    const queryString = new URLSearchParams(data).toString();

    const res: Response = await fetch(
      `http://127.0.0.1:8080/auth/register?${queryString}`,
      {
        method: "POST",
      },
    );

    // if (!res.ok) {
    // }

    const ex = await res.json() as UserRegistration;
    const origin = req.headers.get("origin");

    return new Response("", {
      status: 301,
      headers: {
        Location: "/login",
      },
    });
    // return Response.redirect(`${origin}/login`, 301);
    //
  },
};

export default function Home({ data }: PageProps<Data>) {
  return (
    <>
      <Head>
        <title>Fresh App</title>
      </Head>
      <div class="flex justify-center flex-row justify-items-center w-full">
        <form method="POST" class="flex flex-col">
          <Input
            value={`test-${randomNumber(0, 999999999)}@tste.com`}
            type="email"
            placeholder="email"
            name="email"
          />
          <Input
            type="password"
            placeholder="password"
            name="password"
            value="123456"
          />
          <Input
            type="password"
            placeholder="please repeat your password"
            name="password_repeat"
            value="123456"
          />

          <button type="submit">Check workout</button>
        </form>
      </div>
    </>
  );
}
