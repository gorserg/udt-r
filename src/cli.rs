use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};

pub fn get_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("log_file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Log file name"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("turn on debugging information")
                .short("d"),
        )
        .arg(Arg::with_name("server").help("start server").short("s"))
        .get_matches()
}
