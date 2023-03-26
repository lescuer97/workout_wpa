function ts_type_from_protobuf_type() {
  // copy proto dir to deno project
  let copy_proto = Deno.run({ cmd: ["cp", "-r", "../proto", "."] });

  copy_proto.status().then((status: Deno.ProcessStatus) => {
    if (status.success) {
      console.log("copied the proto dir correctly");
    } else {
      throw Error("something went wrong in the copying");
    }
  });

  try {
    Deno.readDirSync("generated_types");
  } catch (_) {
    Deno.mkdirSync("generated_types");
  }

  // command to run protoc
  const protoc = Deno.run({
    cmd: ["protoc", "--ts_out=generated_types", "proto/workouts.proto"],
  });

  // wait for success to clear the types
  protoc.status().then((status: Deno.ProcessStatus) => {
    if (status.success) {
      let copy_of_types = Deno.run({
        cmd: ["cp", "generated_types/proto/workouts.ts", "generated_types/"],
      });

      Deno.run({ cmd: ["rm", "-r", "generated_types/proto"] });

      copy_of_types.status().then(async (status: Deno.ProcessStatus) => {
        if (status.success) {
          const workoutsFile: string = await Deno.readTextFile(
            "generated_types/workouts.ts",
          );

          const newWorkoutFile = workoutsFile.replaceAll(
            "google-protobuf",
            "npm:google-protobuf",
          );

          Deno.writeTextFile("generated_types/workouts.ts", newWorkoutFile)
            .then(
              () => {
                console.log("Written to file correctly");
              },
            ).catch((err) => {
              throw Error(err);
            });

          console.log("copied the types correctly");
        }
      });
    } else {
      throw Error("something went wrong in the compilation");
    }
  }).then((res) => {
    Deno.run({ cmd: ["rm", "-r", "proto"] });
  });
}

ts_type_from_protobuf_type();

// file_names().then((fileNames) => {
//   console.log(fileNames);
// })
