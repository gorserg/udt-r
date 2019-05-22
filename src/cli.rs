use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ServerType {
    Simple,
    Actix,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub run_server: bool,
    pub server_type: ServerType,
    pub host: String,
    pub port: i32,
    pub debug: bool,
    pub log_file: String,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            run_server: true,
            server_type: ServerType::Actix,
            host: String::from("localhost"),
            port: 3000,
            debug: false,
            log_file: String::from("trace.log"),
        }
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Options {
    pub fn new() -> Self {
        let mut options = Options::default();

        let matches = App::new(crate_name!())
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
                    .help("Turn on debugging information")
                    .short("d"),
            )
            .arg(
                Arg::with_name("server")
                    .long("server")
                    .help("Start server")
                    .short("s"),
            )
            .arg(
                Arg::with_name("host")
                    .long("host")
                    .help(&format!("Server host, default {}", options.host))
                    .short("h"),
            )
            .arg(
                Arg::with_name("port")
                    .long("port")
                    .help(&format!("Server port, default {}", options.port))
                    .short("p"),
            )
            .get_matches();

        if matches.is_present("debug") {
            options.debug = true;
        }

        if matches.is_present("server") {
            options.run_server = true;
        }

        if let Some(host) = matches.value_of("host") {
            options.host = host.to_lowercase();
        }

        options
    }
}
