use std::fs;
use std::io::Result as IoResult;
use std::path::Path;

use remove_dir_all::*;

fn get_path_or_print_help() -> Option<String> {
    let args = std::env::args();
    if args.len() < 2 {
        println!("usage: tmp <path>");
        std::process::exit(1)
    } else {
        args.skip(1).next()
    }
}

fn main() -> IoResult<()> {
    let raw_path = get_path_or_print_help().unwrap();
    let tmp_path = Path::new(&raw_path);
    if tmp_path.exists() {
        println!("removing...");
        remove_dir_all(tmp_path)?;
    }
    println!("creating...");
    fs::create_dir(tmp_path)?;
    Ok(())
}
