use std::env;

fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());

    let args: Vec<String> = env::args().collect();
    let filter = if args.len() > 1 { &args[1] } else { "" };
    assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());

    for _ in 0..100 {
        let string = subscriber.recv_string(0).unwrap().unwrap();
        println!("{}", string);
    }
}
