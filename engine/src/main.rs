fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    println!("Engine: ZeroMQ REP socket listening on tcp://*:5555");

    let mut msg = zmq::Message::new();
    let ack: &[u8] = &[0x06u8];

    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Engine: Received {} bytes", msg.len());

        // Use the slice reference directly as the zmq crate's send method
        // requires a type that implements Sendable, which &[u8] does.
        responder.send(ack, 0).unwrap();
    }
}
