use std::{thread, time::{Duration, SystemTime}, io::Read};
use serialport::{DataBits, StopBits};
use std::io::{self, Write};

fn main() {
    let output: &[u8] = "1".as_bytes();
    let mut serial_buf: Vec<u8> = vec![0; 1000];

    let builder = serialport::new("/dev/ttyACM0", 9600).timeout(Duration::from_millis(3500)).data_bits(DataBits::Eight)
    .stop_bits(StopBits::One);

    sendmsg(&[1_u8]);

    
    // match port.read(serial_buf.as_mut_slice()) {
    //     Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
    //     Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //     Err(e) => eprintln!("{:?}", e)}

    // println!("{:?}", serial_buf[0]);
}


fn sendmsg(input:&[u8]) {
    let builder = serialport::new("/dev/ttyACM0", 9600).timeout(Duration::from_millis(3500)).data_bits(DataBits::Eight)
    .stop_bits(StopBits::One);

    let mut port = builder.open().expect("failed to connect.");


    thread::sleep(Duration::from_secs(3));

    port.write(input).expect("failed to send!");
}