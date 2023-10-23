use core::args_handler::handle_args;
use std::{env, io};

mod core;
mod file;
mod format;
mod media;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Err(err) = handle_args(args.clone()) {
        println!("Error: {}", err);
        return Ok(());
    }

    Ok(())
}
