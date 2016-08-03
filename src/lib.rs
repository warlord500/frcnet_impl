mod controlStructs;
mod crc;

use std::net::{Ipv4Addr, UdpSocket, TcpListener, TcpStream};
use std::mem::{size_of, transmute_copy, transmute};

use controlStructs::{commonControlData2015, robotControl2015};
use crc::generateCRC;

/// this code is rewrite of frc network communcations
/// library from Aardvark-Wpilib
/// this code is written in rust programming
/// language. it is designed to be  simple and straight forwards as it can be.
/// this library is designed to be use concepts  that can easily be translated to c++
/// if neccessary.
extern "C" fn start_thread(team_id: i32) {
    let mut thread_info = frcNetImpl::new();
    thread_info.execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    // lastDynamicPackets : [Vec<u8>; 32],
    ctrl: robotControl2015,
    last_data_packet: controlStructs::commonControlData2015, // dont use semaphores
}

impl frcNetImpl {
    fn new() -> Self {
        frcNetImpl {
            // lastDynamicPackets : [Vec::new(); 32],
            ctrl: robotControl2015::new(),
            last_data_packet: commonControlData2015::blankPack(),
        }
    }


    /// this is the main body of the library
    pub fn execute_thread(&mut self, team_id: i32) {
        let team_id = team_id; //const param.

        // extract the ip of the robot
        //let ip = Ipv4Addr::new(10, (team_id / 100) as u8, (team_id % 100) as u8, 0);
        let ip = Ipv4Addr::new(127,0,0,1);
        let port = 1110;
        let robo_port = 1105;
        let mut recv_buf = [0 as u8; 1024];
        let mut send_buf = [0 as u8; 2048];

        // receive info from driverstations
        let  ds_server = UdpSocket::bind((ip, port)).unwrap();
        ds_server.set_read_timeout(None).unwrap();// always blocking
        println!("binded dsPort");

        let  robot_socket = UdpSocket::bind((ip, robo_port)).unwrap();
        println!("before blocking");
        robot_socket.set_read_timeout(None).unwrap(); // always blocking
        println!("binded robotSocket");
        let enabled = true;

        while enabled {
            // read a message and echo it in reverse back
            let recv_msg =  ds_server.recv_from(&mut recv_buf);
            match recv_msg {
                Err(_) => {println!("packet read failed or different packet format"); }
                Ok(tuple) => {
                    let (size_msg,src) = tuple; // I dont how to do this with less chars!

                    // convert data to packet format
                 //   let data_packet: controlStructs::commonControlData2015 =
                  //      unsafe { transmute_copy(&send_buf) };
                    // Self.readDynamicData(&mut buf);


                    // create packet for send buffer.
                    let send_packet = robotControl2015::generate(&mut self.ctrl);
                    let packet_size = size_of::<robotControl2015>();
                    send_packet.write_to_buf(&mut send_buf[0..packet_size]);


                    let crc = generate_crc32(&mut send_buf);
                    send_buf[packet_size..packet_size+4].copy_from_slice(&unsafe 
                                                                 {transmute::<u32,[u8; 4]>(crc) });
                    let _ = self.add_dynamic_chunks();


                    ds_server.send_to(&mut send_buf[0..(packet_size + 4)], src).unwrap();
                }
            }
        }
    }
    
    /// I am not sure what we do with dynamic data.
    /// k
    /// read the data into buffers and resend the data
    /// I have no idea why I would send the dynamic chunks as
    /// is
    //   fn readDynamicData(&Self, buf :&mut [u8]) -> () {
    // let mut curLoc : usize = std::mem::sizeOf::<commonControlData2015>();
    // let mut sizeOfBlock = Self.last_data_packet.dynaSize;
    // for packets in Self.lastDynamicPackets.into_iter() {
    // packets.clear(); //empty vector
    // packets.reserve(sizeOfBlock);
    // packets.extend_from_slice(&buf[curLoc..sizeOfBlock]);
    //
    // curLoc += sizeOfBlock;
    // sizeOfBlock =  buf[curLoc];
    // }
    // }

     fn add_dynamic_chunks(&self) -> i32 {
        // this is filler function
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::start_thread(9999);
    }
    #[test]
    fn dyna_chunks(){
        use super::*;
        let thread_local = super::frcNetImpl::new();
        assert_eq!(thread_local.add_dynamic_chunks(),1);
    }
}
