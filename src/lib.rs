use std::thread;
use std::net::{Ipv4Addr, UdpSocket, TcpListener, TcpStream};
use std::io::prelude::*;
use std::mem::cp;
use std::mem::sizeOf;
/// this code is rewrite of frc network communcations
/// library from Aardvark-Wpilib
/// this code is written in rust programming
/// language. it is designed to be  simple and straight forwards as it can be.  
/// this library is designed to be use concepts  that can easily be translated to c++
/// if neccessary.
extern "C" fn start_thread(team_id: i32) {
    execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    lastDynamicControlPacket : [Vec<u8>; 32],
    ctrl :  RobotControl2015,
    dynChunks : [dynaChunk; 32]
        //dont use semaphores

}

fn execute_thread(team_id: i32) {
    let team_id = team_id; //const param.

    // extract the ip of the robot
    let ip = Ipv4Addr::new(10,(team_id/100) as u8,(team_id % 100) as u8,0);
    let port = 1110;
    let mut buf = [0 as u8; 1024];

// receive info from driverstations
    let mut dsServer =  UdpSocket::bind((ip,port)).unwrap();
    dsServer.set_read_timeout(None);// always blocking
    println!("binded dsPort");

    let mut robotSocket =  UdpSocket::bind((ip,port)).unwrap();
    robotSocket.set_read_timeout(None);// always blocking
    println!("binded robotSocket");

    while enabled {
       //read a message and echo it in reverse back
       let (sizeMsg,src) = dsServer.recv_from(&mut buf).unwrap(); 
       if (sizeMsg > 0) && (sizeMsg = std::mem::sizeOf::<commonControlData2015>()) {
           println!("read failed or different packet format");
       } else { 
           //convert data to packet format. 
           let dataPacket : commonControlData2015 =  unsafe { mem::transmute_copy(&buf)}; 
           readDynamicData();
           
           let sendPacket = commonControlData2015.generate(&mut self.packetIndex);
           addDynamicChunks(&mut dynChunks,&mut sendBuffer);

           generateCRC(&mut sendbuffer);
           dsServer.send(addr, &mut sendBuffer);
        }
    }


    
    
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::start_thread(9999);
    }
}
