import { Head } from "$fresh/runtime.ts";

export default function Home() {
  return (
    <>
      <Head>
        <title>Fresh App</title>
      </Head>
      <div>
        <h1>Hi!</h1>

        <p>
          please register to the website<a href="/register">Register</a>
        </p>
        <p>
          if you already have and account <a href="/login">Login</a>
        </p>
      </div>
    </>
  );
}
