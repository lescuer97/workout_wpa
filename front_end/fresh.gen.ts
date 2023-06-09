// DO NOT EDIT. This file is generated by fresh.
// This file SHOULD be checked into source version control.
// This file is automatically updated during development when running `dev.ts`.

import config from "./deno.json" assert { type: "json" };
import * as $0 from "./routes/[name].tsx";
import * as $1 from "./routes/_middleware.ts";
import * as $2 from "./routes/api/joke.ts";
import * as $3 from "./routes/create/excersice.tsx";
import * as $4 from "./routes/index.tsx";
import * as $5 from "./routes/login.tsx";
import * as $6 from "./routes/register.tsx";
import * as $$0 from "./islands/Counter.tsx";
import * as $$1 from "./islands/Input.tsx";
import * as $$2 from "./islands/Select.tsx";

const manifest = {
  routes: {
    "./routes/[name].tsx": $0,
    "./routes/_middleware.ts": $1,
    "./routes/api/joke.ts": $2,
    "./routes/create/excersice.tsx": $3,
    "./routes/index.tsx": $4,
    "./routes/login.tsx": $5,
    "./routes/register.tsx": $6,
  },
  islands: {
    "./islands/Counter.tsx": $$0,
    "./islands/Input.tsx": $$1,
    "./islands/Select.tsx": $$2,
  },
  baseUrl: import.meta.url,
  config,
};

export default manifest;
