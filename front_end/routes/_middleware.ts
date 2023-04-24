import { MiddlewareHandlerContext } from "$fresh/server.ts";

export const handler = [
  async function check_if_logged_in(
    req: Request,
    ctx: MiddlewareHandlerContext<State>,
  ) {
    const cookie = req.headers.get("cookie") as string;
    const headers = new Headers();
    headers.set("cookie", cookie);

    const res: Response = await fetch(
      `http://127.0.0.1:8080/auth/checklogin`,
      {
        method: "GET",
        headers,
      },
    );

    ctx.state.goodLogin = res.ok;

    return ctx.next();
  },
];
