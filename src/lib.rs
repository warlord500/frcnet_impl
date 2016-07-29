use std::thread;
use std::net::{Ipv4Addr, UdpSocket, TcpListener, TcpStream};
use std::io::prelude::*;
// const TEAM_ID : i32 = 9999; //default
extern "C" fn start_thread(team_id: i32) {
    execute_thread(team_id.clone());
    println!("start_thread")

}
struct frcNetImpl {
    lastDynamicControlPacket : [Vec<u8>; 32]
}

fn execute_thread(team_id: i32) {
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

    while(enabled) {
        //read a message and echo it in reverse back
        let (sizeMsg,src) = dsServer.recv_from(&mut buf).unwrap(); 
        if sizeMsg < 0 {println!("read failed");}
    }


    
    
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::start_thread(9999);
    }
}
