use std::{thread, time::{Duration, SystemTime}};
use serialport::{DataBits, StopBits, SerialPort};
use std::io::{Write};


fn main() {
    let builder = serialport::new("COM5", 9600) //params to tune for the Arduino port
        .timeout(Duration::from_millis(3000))
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);
        // .flow_control(serialport::FlowControl::Hardware);
        println!("Waiting for next 5-minute interval...");

        setup(10); //starts it on the next 5mins. Accounts for the time it takes for
        // the port to reset as well.

    let mut port = builder.open().expect("failed to connect.");

        
    thread::sleep(Duration::from_millis(2500)); //need to wait 3s for the Arduino port to 
    //reset and communicate. When it's opened here, it will stay open for the lifetime of the
    //variable "port", which is as long as the loop continues to run. 

    loop {
    //below func gets system time.
    let time_startloop:u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut timing_flags: [i32; 3] = [0,0,0]; //flagging var to check when all 3 levels are cycled thru
    let routime:u64 = 20; //var to define the run time in 1 place
    let max_buffer:u64 = 5; //maximum error acccepted before a condition is considered skipped
    
    //second while loop for the interior stuff
    while timing_flags != [1,1,1] { //iterate any time the flags aren't all set

        let time_now:u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if time_now - time_startloop >= 0 
            && time_now-time_startloop <= max_buffer
            && timing_flags[0]!=1 {

                sendmsg(1, &mut port);
                timing_flags[0] = 1;
                println!("Level 1");
                thread::sleep(Duration::from_secs(routime))
            }    

        if time_now - time_startloop >= routime 
        && time_now-time_startloop <= (routime)+max_buffer
        && timing_flags[1]!=1 {

            sendmsg(2, &mut port);
            timing_flags[1] = 1;
            println!("Level 2");
            thread::sleep(Duration::from_secs(routime))
        }
        
        if time_now - time_startloop >= routime*2
        && time_now - time_startloop <=(routime*2)+max_buffer
        && timing_flags[2]!=1 {

            sendmsg(0, &mut port);
            timing_flags[2] = 1;
            println!("Level 3");
            thread::sleep(Duration::from_secs(routime))
        }
        
    }
}
}
    
fn sendmsg(input:u8, port: &mut Box<dyn SerialPort>) {   
    let builder = serialport::new("COM5", 9600) //params to tune for the Arduino port
        .timeout(Duration::from_millis(3000))
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let mut port = builder.open().expect("failed to connect.");  
  
    let binding: String = input.to_string();
    let writebyte:&[u8] = &binding.as_bytes();
    port
        .write(writebyte)
        .expect("failed to send!");
}

fn setup(routime:u64) {
    let mut startflag:u8 = 0;

    while startflag == 0 {
    let time_now:u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if time_now % (routime) == 0 {
        startflag = 1;}
        else {startflag = 0;}
    }
}


