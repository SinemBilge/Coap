extern crate coap;
use coap::{CoAPClient};

fn main() {
    // If we are running in Docker, set the server to `server`
    // Otherwise set it to 127.0.0.1
    let mut server = "127.0.0.1";
    if env!("HOME") == "/root" {
        server = "coap-server";
    }

    let base_url = format!("coap://{}:5683/Rust", server);
    println!("Base URL: {}", base_url);

    for i in 0..19 {
        let url = format!("{}?i={}", base_url, i);
        println!("Client request {}: {}", i, url);

        let response = CoAPClient::get(&url).unwrap();
        println!("Server reply {}: {}", i, String::from_utf8(response.message.payload).unwrap());
    }
    
    loop {}
}
