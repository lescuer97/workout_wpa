import { Head } from "$fresh/runtime.ts";
import {
  HandlerContext,
  Handlers,
  MiddlewareHandlerContext,
} from "$fresh/server.ts";

export const handler: Handlers = {
  GET(req: Request, ctx: HandlerContext<State>) {
    if (!ctx.state.goodLogin) {
      return new Response("", {
        status: 301,
        headers: { location: "/login" },
      });
    }

    return ctx.render();
  },
};
export default function Home() {
  return (
    <>
      <Head>
        <title>Fresh App</title>
      </Head>
      <div>
        <h1>Hi!</h1>
        hi! welcome to my workout app!
      </div>
    </>
  );
}
