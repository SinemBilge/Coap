extern crate coap;
use coap_lite::{RequestType as Method};
use coap::Server;
// use coap_lite::MessageClass::Response;
use tokio::runtime::Runtime;
// use coap::server::Message;
// use coap_lite::MessageType;

fn main() {
    let addr = "0.0.0.0:5683";
	Runtime::new().unwrap().block_on(async move {
        let mut server = Server::new(addr).unwrap();
        println!("Server up on {}", addr);

        server.run(|request| async {
            match request.get_method() {
                &Method::Get => println!("request by get {}", request.get_path()),
                &Method::Post => println!("request by post {}", String::from_utf8(request.message.payload).unwrap()),
                &Method::Put => println!("request by put {}", String::from_utf8(request.message.payload).unwrap()),
                _ => println!("request by other method"),
            };
            return match request.response {
                Some(mut message) => {
                    message.message.payload = b"OK".to_vec();
                    Some(message)
                },
                _ => None
            };
        }).await.unwrap();
    });
}
