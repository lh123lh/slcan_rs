
use slcan::SocketCan;

fn main() {
    let mut can = slcan::CanSocket::new("COM7".to_string(), slcan::BitRate::Setup500Kbit);

    // can.close().unwrap();
    can.open().unwrap();

    loop {
        match can.read() {
            Ok(frame) => println!("{}", frame),
            Err(error) => match error.kind() {
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => (),
                _ => eprintln!("{:?}", error),
            },
        }
    }
}