use std::{thread, 
    time::{Duration, SystemTime}, 
    io::Result,
    path::Path};
use serialport::{DataBits, StopBits};
use chrono::{NaiveDateTime, Datelike, Timelike};


const TIME_OFFSET: u64 = 60*11 + 44; //offset between the NOxWerx and the computer this is running on
const DISPLAYTIME_OFFSET: u64 = 60*60*4; //offset btw EST and UTC (?) 

macro_rules! get_time {
    () => {SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() + TIME_OFFSET
    };
}

macro_rules! get_time_display {
    ($raw_time:ident) => { 
        NaiveDateTime::from_timestamp_opt(
        ($raw_time - DISPLAYTIME_OFFSET) as i64,0_u32)
    .unwrap()
        
    };
}

fn main() {
    println!("Waiting for next 5-minute interval...");
    let file_write_interval:u64 = (5*60*3)*4; //per-hour recording
    let filepath: &Path = Path::new("C:\\Users\\EIS\\Desktop\\valve_flags");
    
    
    setup(5*60); //starts it on the next 5mins. Accounts for the time it takes for\
            // the port to reset as well.
    
    loop {

    let fstarttime:u64 = get_time!();

    let displaystring = format!("{:02}{:02}{:02}{:02}{:02}", //format YYMMDDHHMM
        get_time_display!(fstarttime).year(), 
        get_time_display!(fstarttime).month(),get_time_display!(fstarttime).day(),
        get_time_display!(fstarttime).hour(),get_time_display!(fstarttime).minute()
        );
        
    let filenamestr: String = format!("rob_noxbox_{}.csv", &displaystring); //concatenating the filenamestr
    let file = filepath.join(&filenamestr); //final filename

    while get_time!() - fstarttime < file_write_interval {
    //below func gets system time.
    let time_startloop: u64 = get_time!();
    let mut timing_flags: [i32; 3] = [0,0,0]; //flagging var to check when all 3 levels are cycled thru
    let routime:u64 = 5*60; //var to define the run time in 1 place
    let max_buffer:u64 = 10; //maximum error acccepted before a condition is considered skipped
    
    //second while loop for the interior stuff
    while timing_flags != [1,1,1] { //iterate any time the flags aren't all set

        let time_now:u64 = get_time!();
        
        let displaytime: NaiveDateTime = get_time_display!(time_now);

        if  time_now-time_startloop <= max_buffer
            && timing_flags[0]!=1 {

                match sendmsg(1){
                    Ok(_usize) =>(),
                    Err(_e) => println!("Failed to write!")
                };
                timing_flags[0] = 1;
                println!("Level 1 at {:02}/{:02}/{:02} {:02}:{:02}:{:02}",
                displaytime.year(), displaytime.month(), displaytime.day(),
                displaytime.hour(), displaytime.minute(), displaytime.second());
                _ = write_flags(routime, &file, 1);
            }    

        if time_now - time_startloop >= routime 
        && time_now-time_startloop <= (routime)+max_buffer
        && timing_flags[1]!=1 {

            match sendmsg(2){
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[1] = 1;
            println!("Level 2 at {:02}/{:02}/{:02} {:02}:{:02}:{:02}",
                displaytime.year(), displaytime.month(), displaytime.day(),
            displaytime.hour(), displaytime.minute(), displaytime.second());

            _ = write_flags(routime, &file, 2);
        }
        
        if time_now - time_startloop >= routime*2
        && time_now - time_startloop <=(routime*2 )+max_buffer
        && timing_flags[2]!=1 {

            match sendmsg(0){
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[2] = 1;
            println!("Level 3 at {:02}/{:02}/{:02} {:02}:{:02}:{:02}",
                displaytime.year(), displaytime.month(), displaytime.day(),
            displaytime.hour(), displaytime.minute(), displaytime.second());
            _ = write_flags(routime, &file, 3);
        }

        if time_now - time_startloop >= routime*3 + 15 &&
        timing_flags !=[1,1,1]{
            println!("Overtime at {:02}/{:02}/{:02} {:02}:{:02}:{:02}",
                displaytime.year(), displaytime.month(), displaytime.day(),
            displaytime.hour(), displaytime.minute(), displaytime.second());
            sendmsg(0).expect("failed!");
            continue
        }
        
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

    if time_now % (routime) == 0 {
        startflag = 1;}
        else {startflag = 0;}
    }
}

fn write_flags(routime:u64, 
    file:&Path, position:u8)-> Result<()> {

    let mut wtr = csv::Writer::from_path(file)?; //pass it to the writer function

    let mut counter: u64 = 1; 

    while counter < routime { //write 1 entry per second with the flags.

        let now = get_time!();
        let displaynow = get_time_display!(now);

        wtr.write_record(vec![displaynow.to_string(), position.to_string()])?;
        counter += 1;

        thread::sleep(Duration::from_millis(1000))
    }   
    wtr.flush()?;
    Ok(())
}



