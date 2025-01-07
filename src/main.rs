use std::io;

mod tetridomino;
mod tui;

fn main() -> Result<(), io::Error> {
    let mut term = tui::init()?;

    println!("Hello, world!");

    tui::restore(&mut term)?;

    // TODO: test transforms here

    Ok(())
}