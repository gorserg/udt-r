#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate log4rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;
mod logger;
mod server;
mod sys_info;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // application options
    let options = cli::Options::new();
    // init logger (file + console)
    logger::init(options.log_file.as_str());

    info!("----------------------------------------------------");
    info!("App started");
    info!("Options: {}", options);
    info!("Host info: {}", sys_info::SystemInfo::new());
    if options.run_server {
        server::start(options);
    }
    Ok(())
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let _result = run();
}
