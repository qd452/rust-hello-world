use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:26379").await.unwrap();
    println!("Server started");
    println!("Listening");

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        // process(socket).await;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

#[allow(dead_code)]
async fn process(socket: TcpStream) {
    let mut db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Respond with an error
        // let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
