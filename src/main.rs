mod utils;
mod model;
mod controller;
mod test;

use tiny_http::{Server, Response, StatusCode, Header};
use crate::controller::main_handler;


fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();

        let response_string = main_handler(content, request.url(), request.method().as_str());


        let mut response = Response::from_string(response_string);
        request.respond(response.with_header(Header{ field: "content-type".parse().unwrap(),
            value: "application/json".parse().unwrap()
        })).ok();
    }
}
