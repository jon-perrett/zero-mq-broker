use rand::Rng;

fn main() {
    //
    println!("Connecting to hello world server...");
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester.connect("tcp://127.0.0.1:5555").unwrap();
    let mut rng = rand::thread_rng();

    loop {
        let temperature = rng.gen_range(-80..135);
        let relhumidity = rng.gen_range(10..60);

        let update = format!("{} {}", temperature, relhumidity);
        requester.send(&update, 0).unwrap();
        let mut message = zmq::Message::new();
        requester.recv(&mut message, 0).unwrap();
    }
}
