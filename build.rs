use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

const RESOURCES_DIR: &str = "res/";

fn main() -> Result<()> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed={}*", RESOURCES_DIR);

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push(RESOURCES_DIR);
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
