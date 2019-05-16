extern crate clap;

use clap::{App, AppSettings, Arg};

use ansi_term::Color;
use ansi_term::Color::Fixed;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let result = run();
}
