use crate::cli::{Options, ServerType};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

extern crate actix_web;
use actix_web::{server, http,  HttpRequest, HttpResponse, fs, App};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    stream.read(&mut buf).unwrap();
    let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut parsed_headers);
    req.parse(&buf).unwrap();
    match req.path {
        Some(ref path) => {
            let mut page = "/index.html";
            if !path.ends_with("/") {
                page = path;
            }
            let body = std::fs::read_to_string(format!("static{}", page)).unwrap_or_default();
            let status = "HTTP/1.1 200 OK\r\n".to_string();
            let header = status + "Content-Type: text/html; charset=UTF-8\r\n\r\n";
            let res = header + &body + "\r\n";
            let data = res.as_bytes();
            stream.write(data).expect("Can't write response");
        }
        None => {}
    }

    // match stream.read(&mut buf) {
    //     Ok(_) => {
    //         //let req_str = String::from_utf8_lossy(&buf);
    //         //req.parse(&buf).unwrap();
    //         req.parse(b);
    //         //println!("{}", req_str);
    //     }
    //     Err(e) => println!("Unable to read stream: {}", e),
    // }
}

// fn handle_write(mut stream: TcpStream) {
//     let contents =
//         fs::read_to_string("static/index.html").expect("Something went wrong reading the file");

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n",
//         contents
//     );
//     match stream.write(response.as_bytes()) {
//         Ok(_) => println!("Response sent"),
//         Err(e) => println!("Failed sending response: {}", e),
//     }
// }

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    //handle_write(stream);
}


fn index(_req: &HttpRequest) -> &'static str {
    "Main app"
}
fn app1(_req: &HttpRequest) -> &'static str {
    "From app1"
}
fn app2(_req: &HttpRequest) -> &'static str {
    "From app2"
}

pub fn start(options: Options) {
    // simple server
    if options.server_type == ServerType::Simple {
        let listener = TcpListener::bind(format!("{}:{}", options.host, options.port)).unwrap();
        println!(
            "Listening for connections on {}:{}",
            options.host, options.port
        );

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
    } else if options.server_type == ServerType::Actix {
        info!("Start actix");
         let app = App::new()
             .resource("/", |r| r.f(index));
            //  .handler(
            //      "/static",
            //      fs::StaticFiles::new("./static")
            //          .unwrap()
            //          .show_files_listing()        
            //  ).finish();

        let server = server::new(|| {
            vec![
                App::new()
                    .prefix("/app1")
                    .resource("/", |r| r.f(app1)),
                App::new()
                    .prefix("/app2")
                    .resource("/", |r| r.f(app2)),
                App::new()
                    .handler(
                        "/static",
                        fs::StaticFiles::new("./static")
                            .unwrap()
                            .show_files_listing()),
                App::new().resource("/", |r| r.f(index)),
            ]
        });

        server
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();

           info!("Stop actix");
    }
}
