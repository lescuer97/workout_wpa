# Workout sessions WPA

This is a simple WPA made with Fresh(Deno) and Actix(rust) on the server. its
just for categorizing my workouts and ordering them.

## Developing

Code is separed in two directories:

```
front_end/
server/
```

##### Running the front-end:

```
cd fron_end
deno task start
```

##### Running dev server:

```
cd server
# you can use cargo watch
cargo watch -x run
```

###### Notes: momentary problems with protobuf types

Fresh is not supporting
[npm prefix](https://github.com/denoland/fresh/issues/978) at the moment. this
is necessary for the protobuf actions to work. for now the types are going to be
sent via JSON instead of protobuf and are going to be directtly typed to Deno
instead of being generated.

- On the server there is and alternative way to do this with protobug with actix
  called
  [actix-protobuf](https://github.com/actix/actix-extras/tree/master/actix-protobuf)

- I added a Cargo.toml file to the root of directory for the workspace at first
  because the precommit-hooks where not working correctly
