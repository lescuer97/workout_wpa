import { Head } from "$fresh/runtime.ts";
import { Handlers, PageProps } from "$fresh/server.ts";

import Input from "@/islands/Input.tsx";

interface Data {
  results: UserLogin;
  query: string;
}

export const handler: Handlers<Data> = {
  async POST(req: Request, ctx) {
    const data: FormData = await req.formData();

    const queryString = new URLSearchParams(data).toString();

    const res: Response = await fetch(
      `http://127.0.0.1:8080/auth/login?${queryString}`,
      {
        method: "POST",
      },
    );

    // if (!res.ok) {
    // }

    const cookie = res.headers.get("set-cookie") as string;
    const headers = new Headers();
    headers.set("set-cookie", cookie);
    headers.set("Location", "/");

    return new Response("", {
      status: 301,
      headers,
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
            value={`test22s@test.com`}
            type="email"
            placeholder="email"
            name="email"
          />
          <Input
            type="password"
            placeholder="password"
            name="password"
            value="&#8V*n%!WL5^544#Z7xr"
          />

          <button type="submit">Check workout</button>
        </form>
      </div>
    </>
  );
}
