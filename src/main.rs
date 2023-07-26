use std::{thread, time::{Duration, SystemTime}, io::Result};
use serialport::{DataBits, StopBits};

const TIME_OFFSET: u64 = 60*11 + 44;

fn main() {
        println!("Waiting for next 15-minute interval...");
        setup(5*60); //starts it on the next 5mins. Accounts for the time it takes for
        // the port to reset as well.
    loop {
    //below func gets system time.
    let time_startloop:u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() + TIME_OFFSET;


    let mut timing_flags: [i32; 3] = [0,0,0]; //flagging var to check when all 3 levels are cycled thru
    let routime:u64 = 5*60; //var to define the run time in 1 place
    let max_buffer:u64 = 10; //maximum error acccepted before a condition is considered skipped
    
    //second while loop for the interior stuff
    while timing_flags != [1,1,1] { //iterate any time the flags aren't all set

        let time_now:u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() + TIME_OFFSET;

        if  time_now-time_startloop <= max_buffer
            && timing_flags[0]!=1 {

                match sendmsg(1){
                    Ok(_usize) =>(),
                    Err(_e) => println!("Failed to write!")
                };
                timing_flags[0] = 1;
                println!("Level 1");
                thread::sleep(Duration::from_secs(routime))
            }    

        if time_now - time_startloop >= routime 
        && time_now-time_startloop <= (routime)+max_buffer
        && timing_flags[1]!=1 {

            match sendmsg(2){
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[1] = 1;
            println!("Level 2");
            thread::sleep(Duration::from_secs(routime))
        }
        
        if time_now - time_startloop >= routime*2
        && time_now - time_startloop <=(routime*2)+max_buffer
        && timing_flags[2]!=1 {

            match sendmsg(0){
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[2] = 1;
            println!("Level 3");
            thread::sleep(Duration::from_secs(routime))
        }

        if time_now - time_startloop >= routime*3 + 15 &&
        timing_flags !=[1,1,1]{
            println!("Overtime!");
            sendmsg(0).expect("failed!");
            continue
        }
        
    }
}
}
    
fn sendmsg(input:u8) -> Result<usize> {    //port: 
   
    let builder = serialport::new("COM5", 9600) //params to tune for the Arduino port
        .timeout(Duration::from_millis(3000))
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let mut port = builder.open().expect("failed to connect.");

    
    let binding: String = input.to_string();
    let writebyte:&[u8] = &binding.as_bytes();

    port
    .write(writebyte)

}
        
fn setup(routime:u64) {
    let mut startflag:u8 = 0;

    while startflag == 0 {
    let time_now:u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()+TIME_OFFSET;

    if time_now % (routime*3) == 0 {
        startflag = 1;}
        else {startflag = 0;}
    }
}

