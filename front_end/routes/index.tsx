import { Head } from "$fresh/runtime.ts";
import Button_send from "@/islands/Button_send.tsx";

export default function Home() {
  return (
    <>
      <Head>
        <title>Fresh App</title>
      </Head>
      <div>
        <Button_send />
      </div>
    </>
  );
}
