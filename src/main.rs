extern crate clap;

mod cli;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_matches();

    println!(
        "Debugging mode is: {}",
        if matches.is_present("debug") {
            "ON"
        } else {
            "OFF"
        }
    );

    match matches.subcommand() {
        ("test", Some(sub_matches)) => {
            println!("{:}", sub_matches.usage());
            Ok(())
        }
        _ => {
            println!("{:}", matches.usage());
            Ok(())
        }
    }
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let result = run();
}
