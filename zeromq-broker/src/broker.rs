fn main() {
    let context = zmq::Context::new();
    let receive = create_socket(&context, zmq::REP);
    let publish = create_socket(&context, zmq::PUB);
    bind_socket(&receive, 5555);
    bind_socket(&publish, 5556);
    let items = &mut [receive.as_poll_item(zmq::POLLIN)];

    loop {
        zmq::poll(items, -1).unwrap();
        if items[0].is_readable() {
            match receive.recv_msg(0) {
                Ok(msg) => {
                    publish.send(msg, 0).unwrap();
                    let _ = receive.send("OK", 0);
                }
                Err(_) => {
                    println!("Couldn't receive message");
                    receive.send("NOK", 0).unwrap();
                }
            };
        }
    }
}

fn bind_socket(socket: &zmq::Socket, port: i32) {
    match socket.bind(&format!("tcp://127.0.0.1:{}", port)) {
        Ok(_) => println!("Bound to port {}", port),
        Err(_) => {
            println!("Failed to bind socket");
            panic!()
        }
    };
}

fn create_socket(context: &zmq::Context, socket_type: zmq::SocketType) -> zmq::Socket {
    match context.socket(socket_type) {
        Ok(socket) => {
            println!("Successfully created socket.");
            return socket;
        }
        Err(_) => {
            println!("Failed to create socket.");
            panic!()
        }
    };
}
