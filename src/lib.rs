mod out_packet;
mod crc;
mod send;
mod interface;

use out_packet::commonControlData2015;
use send::robot_control_2015;

/// this code is rewrite of frc network communcations
/// library from Aardvark-Wpilib
/// this code is written in rust programming
/// language. it is designed to be  simple and straight forwards as it can be.
/// this library is designed to be use concepts  that can easily be translated to c++
/// if neccessary.
pub struct frcNetImpl {
    // lastDynamicPackets : [Vec<u8>; 32],
    ctrl: robot_control_2015,
    last_data_packet: out_packet::commonControlData2015, // dont use semaphores
}

impl frcNetImpl {
    pub fn new() -> Self {
        frcNetImpl {
            // lastDynamicPackets : [Vec::new(); 32],
            ctrl: robot_control_2015::new(),
            last_data_packet: commonControlData2015::blankPack(),
        }
    }


    /// this is the main body of the library
    /// it contains basic loop code for sending network packets. 
    pub fn execute_thread(&mut self, team_id: i32) {
        use std::net::{Ipv4Addr, UdpSocket};
        use std::mem::{size_of, transmute_copy, transmute};
        use crc::generate_crc32;

        let team_id = team_id; //const param.

        // extract the ip of the robot
        let ip = Ipv4Addr::new(10, (team_id / 100) as u8, (team_id % 100) as u8, 0);
        let port = 1110;
        let robo_port = 1105;
        let mut recv_buf = [0 as u8; 1024];
        let mut send_buf = [0 as u8; 2048];

        // receive info from driverstations
        let  ds_server = UdpSocket::bind((ip, port)).unwrap();
        ds_server.set_read_timeout(None).unwrap();// always blocking
        println!("binded dsPort");

        let  robot_socket = UdpSocket::bind((ip, robo_port)).unwrap();
        robot_socket.set_read_timeout(None).unwrap(); // always blocking
        println!("binded robotSocket");


        loop {
            // read a message and echo it in reverse back
            let recv_msg =  ds_server.recv_from(&mut recv_buf);
            match recv_msg {
                Err(_) => {println!("packet read failed or different packet format"); }
                Ok(tuple) => {
                    let (_,src) = tuple; //extract src of message

                    //convert latest packet to structure for analyzing
                    self.last_data_packet = unsafe { transmute_copy(&send_buf) };

                    // create packet for send buffer.
                    let send_packet = robot_control_2015::generate(&mut self.ctrl);
                    let packet_size = size_of::<robot_control_2015>();
                    send_packet.write_to_buf(&mut send_buf[0..packet_size]);


                    let crc = generate_crc32(&send_buf);//write crc to buffer
                    send_buf[packet_size..packet_size+4].copy_from_slice(
                                        unsafe{
                                                 &transmute::<u32,[u8; 4]>(crc) 
                                        });


                    ds_server.send_to(&mut send_buf[0..(packet_size + 4)], src).unwrap();
                } // end of Ok
            } // end of match
        } // end of loop
    }
}

#[cfg(test)]
mod tests {
    /*#[test]
    fn it_works() {
        super::start_thread(9999);
    } */


}
