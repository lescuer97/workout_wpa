const path_to_proto = '../proto' as const;


// return list of file names
 async function file_names(): Promise<string[]> {
const fileNames: string[] = [];
  
  for await (const dirEntry of Deno.readDir(path_to_proto)) {
    if (dirEntry.isFile) {
      fileNames.push(dirEntry.name);
    }
  }

  return fileNames;
 }
    

function regex_change_to_deno_import() {
    
}




function ts_type_from_protobuf_type() {
    //check if dir doesn't exists else create it create directory if not exists
    if (!Deno.readDirSync("generated_types")) {
        Deno.mkdirSync("generated_types");
    }
    // command to run protoc
    Deno.run({ cmd: ["protoc", "--ts_out=generated_types", "../proto/*.proto"] });



}
ts_type_from_protobuf_type();

file_names().then((fileNames) => {
  console.log(fileNames);
})
