use std::{io::Result};

use fs_extra::dir::*;

fn main() -> Result<()> {

    let mut options = CopyOptions::new();
    options.overwrite = true;

    fs_extra::dir::copy("../proto", "src/", &options).unwrap();

    prost_build::compile_protos(&["proto/workouts.proto"], &["src"])?;

    fs_extra::dir::remove("src/proto").unwrap();

    Ok(())
}
