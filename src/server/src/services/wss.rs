use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

pub fn start_server(addr: &str, port: u16) {
    let server = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
    tokio::spawn(async move {
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let req = websocket.read().unwrap();

                    let res = match req {
                        Message::Text(text) => {
                            println!("{:?}", text);
                            Some(Message::Text(text))
                        }
                        Message::Binary(bytes) => {
                            println!("{:?}", bytes);
                            // TODO Define response here.
                            let msg = Message::Binary(bytes);
                            Some(msg)
                        }
                        _ => None,
                    };
                    if let Some(res) = res {
                        websocket.send(res).unwrap();
                    }
                }
            });
        }
    });
}
