extern crate std;
use std::net::{Ipv4Addr, UdpSocket, TcpListener, TcpStream};
use std::mem::{sizeOf,transmute_copy,transmute};
mod controlStructs;
mod crc;
/// this code is rewrite of frc network communcations
/// library from Aardvark-Wpilib
/// this code is written in rust programming
/// language. it is designed to be  simple and straight forwards as it can be.  
/// this library is designed to be use concepts  that can easily be translated to c++
/// if neccessary.
extern "C" fn start_thread(team_id: i32) {
    let threadInfo = frcNetImpl {};
    threadInfo.execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    lastDynamicControlPacket : [Vec<u8>; 32],
    ctrl :  RobotControl2015,
    dynChunks : [dynaChunk; 32]
        //dont use semaphores

}

impl frcNetImpl {
    fn execute_thread(team_id: i32) {
        let team_id = team_id; //const param.

        // extract the ip of the robot
        let ip = Ipv4Addr::new(10,(team_id/100) as u8,
                (team_id % 100) as u8,0);
        let port = 1110;
        let mut recvBuf = [0 as u8; 1024];
        let mut sendBufer = [0 as u8; 2048];

    // receive info from driverstations
        let mut dsServer =  UdpSocket::bind((ip,port)).unwrap();
        dsServer.set_read_timeout(None);// always blocking
        println!("binded dsPort");

        let mut robotSocket =  UdpSocket::bind((ip,port)).unwrap();
        robotSocket.set_read_timeout(None);// always blocking
        println!("binded robotSocket");

        while enabled {
           //read a message and echo it in reverse back
           let (sizeMsg,src) = dsServer.recv_from(&mut recvBuf).unwrap(); 
           if (sizeMsg < 0) 
           && (sizeMsg == sizeOf::<controlStruct::commonControlData2015>()) {
               println!("read failed or different packet format");
           } else { 
               //convert data to packet format. 
               let dataPacket : controlStruct::commonControlData2015 = 
                   unsafe { transmute_copy(&buf[0..sizeMsg])}; 
               self.readDynamicData(&mut buf);
               
               //create packet for send buffer.
               let sendPacket = commonControlData2015.generate(&mut self);
               let packetSize = sizeOf::<controlStruct::commonControlData2015>();
               sendBuf[0..packetSize].clone_from_slice(unsafe {mem::transmute(sendPacket) });
               


                let crc = generateCRC(&mut sendbuffer);
                
                sendBuf[packetSize..4].clone_from_slice(unsafe{mem::transmute(crc)});

               dsServer.send(addr, &mut sendBuffer[0..(packetSize+4)]);

            }
        }
    }
    ///I am not sure what we do with dynamic data.
    /// as far as I can tell  we attempt 
    /// read the data into buffers and resend the data
    /// I have no idea why I would send the dynamic chunks as 
    /// is
    fn readDynamicData(&self, buf :&mut [u8]) -> () {
    let mut curLoc = std::mem::sizeOf::<commonControlData2015>();
    let mut sizeOfblock = self.lastDataPacket.dynaSize;
    for packets in self.lastDynamicPackets {
       *packets.clear(); //empty vector
       *packets.reserve(sizeOfBlock);
       *packets.extend_from_slice(buf [curLoc..sizeOfBlock]);

       curLoc += sizeOfBlock;
       sizeOfBlock =  buf[curLoc];
      }  
    }

    fn addDynamicChunks(&self) -> i32 {/*this is filler function*/ }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::start_thread(9999);
    }
}
