use remove_dir_all::ensure_empty_dir;

fn get_path_or_print_help() -> Option<String> {
    let args = std::env::args();
    if args.len() < 2 {
        println!("usage: tmp <path>");
        std::process::exit(1)
    } else {
        args.skip(1).next()
    }
}

fn main() -> std::io::Result<()> {
    let path = get_path_or_print_help().unwrap();
    ensure_empty_dir(path)?;
    Ok(())
}
