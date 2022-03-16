mod utils;
mod model;

use tiny_http::{Server, Response};


fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        
        println!("received request! method: {:?}, url: {:?}, headers: {:?}, body: {:?}",
            request.method(),
            request.url(),
            request.headers(),
            content
        );

        let response = Response::from_string("hello world");
        request.respond(response).ok();
    }
}
