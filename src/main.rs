use std::{thread, 
    time::{Duration, SystemTime}, 
    io::Result};
use serialport::{DataBits, StopBits};
use chrono::{NaiveDateTime, Datelike, Timelike};


pub const TIME_OFFSET: u64 = 60*11 + 44; //offset between the NOxWerx and the computer this is running on
pub const DISPLAYTIME_OFFSET: u64 = 60*60*4; //offset btw EST and UTC (?) 

macro_rules! get_time { //macro gets the current system time and converts it into a seconds figure
    () => {SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() + TIME_OFFSET
    };
}

macro_rules! get_time_display { //takes a time object (even get_time!() from above) and converts it to a human-readable datetime obj
    ($raw_time:ident) => { 
        NaiveDateTime::from_timestamp_opt(
        ($raw_time - DISPLAYTIME_OFFSET) as i64,0_u32)
    .unwrap()
        
    };
}

macro_rules! timestamp { //must be used with a NativeDateTime object to behave as expected
    ($level:literal, $time_obj:ident) => {
        println!("{}, {:02}/{:02}/{:02} {:02}:{:02}:{:02}", $level,
                    $time_obj.year(), $time_obj.month(), $time_obj.day(),
                    $time_obj.hour(), $time_obj.minute(), $time_obj.second())
    };
}

pub fn main() {
    println!("Level, Datetime");
        
    setup(60*5); //starts it on the next 5mins. Accounts for the time it takes for\
            // the port to reset as well.
    


    loop { //this is the top-level loop for the program

    
        

//this is the loop the valve switching operates in 
    //below func gets system time.
    let time_startloop: u64 = get_time!();
    let mut timing_flags: [i32; 3] = [0,0,0]; //flagging var to check when all 3 levels are cycled thru
    let routime:u64 = 60*5; //var to define the run time in 1 place
    let max_buffer:u64 = 25; //maximum error acccepted before a condition is considered skipped
    
    //second while loop for the interior stuff
    while timing_flags != [1,1,1] { //iterate any time the flags aren't all set

        let time_now:u64 = get_time!(); //get time to compare to the loop start time
        
        let displaytime: NaiveDateTime = get_time_display!(time_now); //make a display-friendly version 

        if  time_now-time_startloop <= max_buffer //criteria for the first level
            && timing_flags[0]!=1 {

                match sendmsg(1){ //either send the message successfully or return an error 
                    Ok(_usize) =>(),
                    Err(_e) => println!("Failed to write!")
                };
                timing_flags[0] = 1; //update the timing flag 
                timestamp!(1, displaytime); //print out the time that it switched at 
                thread::sleep(Duration::from_millis(routime*1000)) //sleep for 5min worth of milliseconds
            }    

        if time_now - time_startloop >= routime //criteria for the 2nd valve switch
        && time_now-time_startloop <= (routime)+max_buffer
        && timing_flags[1]!=1 {

            match sendmsg(2){ //same sequence as before
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[1] = 1;
            timestamp!(2, displaytime);
            thread::sleep(Duration::from_millis(routime*1000))
        }
        
        if time_now - time_startloop >= routime*2
        && time_now - time_startloop <=(routime*2 )+max_buffer
        && timing_flags[2]!=1 {

            match sendmsg(0){
                Ok(_usize) =>(),
                Err(_e) => println!("Failed to write!")
            };
            timing_flags[2] = 1;
            timestamp!(3, displaytime);
            thread::sleep(Duration::from_millis(routime*1000))
        }

        if time_now - time_startloop >= routime*3 + 15 &&
        timing_flags !=[1,1,1]{
            timestamp!("overtime", displaytime);
            sendmsg(0).expect("failed!");
            break
        }
        
    }
}
    }

    
pub fn sendmsg(input:u8) -> Result<usize> {    
   //!
   //! # Send Message
   //! 
   //! this function sends the Arduino a message. Because the wait times between messages are so long,
   //! the function is made so that it opens a new serial connection every time it needs to talk to the arduino.
   //! If sending more regular signals, you need to modify this so that the port sits open (probably at the top of the loop)
   //! because the arduino has about a ~2.5 second lag between when you start opening the serial port and when it can
   //! take messages. There is also some timeout on the computer's side of the connection, so if the input is 
   //! slow enough you'll run up against that constraint and have to open the port every time. 


    let builder = serialport::new("COM5", 9600) //params to tune for the Arduino port
        .timeout(Duration::from_millis(3000)) //3s timeout 
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let mut port = builder.open().expect("failed to connect."); //

    
    let binding: String = input.to_string();
    let writebyte:&[u8] = binding.as_bytes();

    port
    .write(writebyte)

}
        
pub fn setup(routime:u64) { 

    //! # Setup
    //! 
    //! this func waits the specified amount of time before starting the rest of the program. Very small wrapper. 

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

// pub fn write_flags(file:&Path, position:u8)-> Result<()> {

//     //! # Write Flags
//     //! 
//     //! This wraps the functionality of writing flags out to a file so that
//     //! it's more concise to read. Requires a system-valid filepath and a position on the valve
//     //! switcher to write out. 

//     let mut wtr = csv::Writer::from_path(file)?; //pass it to the writer function

//         let now = get_time!(); // grabs a time and turns it to a displaytime for writing
//         let displaynow = get_time_display!(now);

//         wtr.write_record(vec![displaynow.to_string(), position.to_string()])?; //try to write out the vector
//     wtr.flush()?; //flush the buffer before closing
//     Ok(()) //if it all worked then return ok
// }

// #[cfg(test)]



// mod tests {

//     use std::{thread, 
//         time::{Duration, SystemTime}, 
//         io::Result,
//         path::Path};
//     use serialport::{DataBits, StopBits};
//     use chrono::{NaiveDateTime, Datelike, Timelike};

//     const TIME_OFFSET: u64 = 60*11 + 44; //offset between the NOxWerx and the computer this is running on
//     const DISPLAYTIME_OFFSET: u64 = 60*60*4; //offset btw EST and UTC (?) 

//     #[test]
//     fn macro_check(){
//         let time = get_time!();
//         let displaytime = get_time_display!(time);
//         let level = 1;
//         assert!(
//             timestamp!(1,displaytime) == 
//             format!("Level {} at {:02}/{:02}/{:02} {:02}:{:02}:{:02}", level,
//                     displaytime.year(), displaytime.month(), displaytime.day(),
//                     displaytime.hour(), displaytime.minute(), displaytime.second())
//         );
//     }

// }