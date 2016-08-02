mod controlStructs;
mod crc;

use std::net::{Ipv4Addr, UdpSocket, TcpListener, TcpStream};
use std::mem::{sizeOf,transmute_copy,transmute};

use controlStructs::{commonControlData2015,robotControl2015};
use crc::generateCRC;

/// this code is rewrite of frc network communcations
/// library from Aardvark-Wpilib
/// this code is written in rust programming
/// language. it is designed to be  simple and straight forwards as it can be.  
/// this library is designed to be use concepts  that can easily be translated to c++
/// if neccessary.
extern "C" fn start_thread(team_id: i32) {
        

    
    threadInfo.execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    //lastDynamicPackets : [Vec<u8>; 32],
    ctrl :  RobotControl2015,
    lastDataPacket : controlStructs::commonControlData,
        //dont use semaphores
}

impl frcNetImpl {
    fn new() -> Self {
        frcNetImpl {
           // lastDynamicPackets : [Vec::new(); 32],
            ctrl : robotControl2015::new(),
            lastDataPacket : commonControlData2015::blankPack(),
         }
    }


    ///this is the main body of the library 
    fn execute_thread(&self ,team_id: i32) {
        let team_id = team_id; //const param.

        // extract the ip of the robot
        let ip = Ipv4Addr::new(10,(team_id/100) as u8,
                (team_id % 100) as u8,0);
        let port = 1110;
        let mut recvBuf = [0 as u8; 1024];
        let mut sendBuffer = [0 as u8; 2048];

    // receive info from driverstations
        let mut dsServer =  UdpSocket::bind((ip,port)).unwrap();
        dsServer.set_read_timeout(None);// always blocking
        println!("binded dsPort");

        let mut robotSocket =  UdpSocket::bind((ip,port)).unwrap();
        robotSocket.set_read_timeout(None);// always blocking
        println!("binded robotSocket");
        let enabled = true;

        while enabled {
           //read a message and echo it in reverse back
           let (sizeMsg,src) = dsServer.recv_from(&mut recvBuf).unwrap(); 
           if (sizeMsg < 0) 
           && (sizeMsg == sizeOf::<controlStructs::commonControlData2015>()) {
               println!("read failed or different packet format");
        
               //convert data to packet format. 
               let dataPacket : controlStructs::commonControlData2015 = 
                   unsafe { transmute_copy(&sendBuffer)}; 
              // Self.readDynamicData(&mut buf);
               

               //create packet for send buffer.
               let sendPacket = commonControlData2015::generate(&mut self);
               let packetSize = sizeOf::<controlStructs::commonControlData2015>();
               sendBuffer[0..packetSize].clone_from_slice(unsafe {transmute(sendPacket) });
               
               
                let crc = generateCRC(&mut sendBuffer);
                sendBuffer[packetSize..4].clone_from_slice(unsafe{transmute(crc)});


               dsServer.send_to(&mut sendBuffer[0..(packetSize+4)],src);
            }
        }
    }
    ///I am not sure what we do with dynamic data.
    /// k
    /// read the data into buffers and resend the data
    /// I have no idea why I would send the dynamic chunks as 
    /// is
 /*   fn readDynamicData(&Self, buf :&mut [u8]) -> () {
        let mut curLoc : usize = std::mem::sizeOf::<commonControlData2015>();
        let mut sizeOfBlock = Self.lastDataPacket.dynaSize;
        for packets in Self.lastDynamicPackets.into_iter() {
           packets.clear(); //empty vector
           packets.reserve(sizeOfBlock);
           packets.extend_from_slice(&buf[curLoc..sizeOfBlock]);

           curLoc += sizeOfBlock;
           sizeOfBlock =  buf[curLoc];
        }  
    } */

    fn addDynamicChunks(&Self) -> i32 {/*this is filler function*/ }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::start_thread(9999);
    }
}
