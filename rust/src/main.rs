use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
  println!("STARTED");
  loop {
    let socket = UdpSocket::bind("127.0.0.1:59999")?;
    let mut buf: [u8; 20] = [0; 20];

    let mut result: Vec<u8> = Vec::new();
    match socket.recv_from(&mut buf) {
      Ok((number_of_bytes, src)) => {
        result = Vec::from(&buf[0..number_of_bytes]);
        match socket.send_to("HELLO JAVASCRIPT".as_bytes(), &src) {
          Ok(_) => (),
          Err(fail) => println!("Error {}", fail),
        }
      }
      Err(fail) => println!("failed listening {:?}", fail),
    }
    let display_result = result.clone();
    let result_str = String::from_utf8(display_result).unwrap();
    println!("received message: {:?}", &result_str);
  }
}
