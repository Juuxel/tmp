use std::fs;
use std::io::Result as IoResult;
use std::path::Path;

use remove_dir_all::*;

fn main() -> IoResult<()> {
    let tmp_path = Path::new("A:/tmp");
    if tmp_path.exists() {
        println!("removing...");
        remove_dir_all(tmp_path)?;
    }
    println!("creating...");
    fs::create_dir(tmp_path)?;
    Ok(())
}
