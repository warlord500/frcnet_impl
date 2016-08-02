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
    let mut threadInfo = frcNetImpl::new();
    threadInfo.execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    // lastDynamicPackets : [Vec<u8>; 32],
    ctrl: robotControl2015,
    lastDataPacket: controlStructs::commonControlData2015, // dont use semaphores
}

impl frcNetImpl {
    fn new() -> Self {
        frcNetImpl {
            // lastDynamicPackets : [Vec::new(); 32],
            ctrl: robotControl2015::new(),
            lastDataPacket: commonControlData2015::blankPack(),
        }
    }


    /// this is the main body of the library
    fn execute_thread(&self, team_id: i32) {
        let team_id = team_id; //const param.

        // extract the ip of the robot
        //let ip = Ipv4Addr::new(10, (team_id / 100) as u8, (team_id % 100) as u8, 0);
        let ip = Ipv4Addr::new(127,0,0,1);
        let port = 1110;
        let roboPort = 1105;
        let mut recvBuf = [0 as u8; 1024];
        let mut sendBuffer = [0 as u8; 2048];

        // receive info from driverstations
        let mut dsServer = UdpSocket::bind((ip, port)).unwrap();
        dsServer.set_read_timeout(None).unwrap();// always blocking
        println!("binded dsPort");

        let mut robotSocket = UdpSocket::bind((ip, roboPort)).unwrap();
        println!("before blocking");
        robotSocket.set_read_timeout(None).unwrap();// always blocking
        println!("binded robotSocket");
        let enabled = true;

        while enabled {
            // read a message and echo it in reverse back
            let (size_msg, src) = dsServer.recv_from(&mut recvBuf).unwrap();
            if (size_msg < 0) {
                println!("read failed or different packet format");
            } else { 

                // convert data to packet format
                let dataPacket: controlStructs::commonControlData2015 =
                    unsafe { transmute_copy(&sendBuffer) };
                // Self.readDynamicData(&mut buf);


                // create packet for send buffer.
                let sendPacket: commonControlData2015 = commonControlData2015::generate();
                //let packetSize = size_of::<commonControlData2015>();
                //sendPacket.write_to_buf(&mut sendBuffer[0..packetSize]);


                //let crc = generateCRC(&mut sendBuffer);
                //sendBuffer[packetSize..4].copy_from_slice(unsafe {&transmute::<i32,[u8; 4]>(crc) });


                //dsServer.send_to(&mut sendBuffer[0..(packetSize + 4)], src);
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
    // let mut sizeOfBlock = Self.lastDataPacket.dynaSize;
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
}
