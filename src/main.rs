use std::{thread, time::{Duration, SystemTime}, io::Read};
use time::macros::datetime;
use serialport::{DataBits, StopBits};
use std::io::{self, Write};

fn main() {

    let builder = serialport::new("/dev/ttyACM0", 9600).timeout(Duration::from_millis(3500)).data_bits(DataBits::Eight)
    .stop_bits(StopBits::One);

    setup(); //starts it on the next 5mins.

    while true {
    //below func gets system time.
    let time_startloop:u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
    .as_secs();

    let mut timing_flags: [i32; 3] = [0,0,0];
    
    //second while loop for the interior stuff
    while timing_flags != [1,1,1] {
        let time_now:u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs();

        if time_now - time_startloop == 0 {
            sendmsg(1);
            timing_flags[0] = 1;
            thread::sleep(Duration::from_millis(5*60*1000));
        }    

        if time_now - time_startloop == 5*60 {
            sendmsg(2);
            timing_flags[1] = 1;
            thread::sleep(Duration::from_millis(5*60*1000));
        }
        
        if time_now - time_startloop == 10*60 {
            sendmsg(0);
            timing_flags[2] = 1;
            thread::sleep(Duration::from_millis(5*60*1000));
        }

    }
}
}
    
    // match port.read(serial_buf.as_mut_slice()) {
    //     Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
    //     Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //     Err(e) => eprintln!("{:?}", e)}

    // println!("{:?}", serial_buf[0]);



fn sendmsg(input:u8) {

    let builder = serialport::new("/dev/ttyACM0", 9600).timeout(Duration::from_millis(3500)).data_bits(DataBits::Eight)
    .stop_bits(StopBits::One);

    let mut port = builder.open().expect("failed to connect.");
    
    let binding: String = input.to_string();
    let writebyte:&[u8] = &binding.as_bytes();
    thread::sleep(Duration::from_secs(3));

    port.write(writebyte).expect("failed to send!");
}

fn setup() {
    let mut startflag:u8 = 0;

    while startflag == 0 {
    let time_now:u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
    .as_secs();
    if time_now % (5*60) == 0 {
        startflag = 1;}
        else {startflag = 0;}
    }
}
