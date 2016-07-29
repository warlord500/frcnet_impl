use std::thread;
use std::net::{Ipv4Addr,UdpSocket,TcpListener,TcpStream};
use std::io::prelude::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
	super::start_thread(9999);
    }
}
//const TEAM_ID : i32 = 9999; //default
extern fn start_thread(team_id : i32){
    //thread::spawn(move ||{}); 
execute_thread(team_id.clone());
    println!("start_thread")

}
fn execute_thread(team_id :i32){
// extract the ip of the robot
   // let ip = Ipv4Addr::new(10,(team_id/100) as u8,(team_id % 100) as u8,15);
   let ip = Ipv4Addr::new(0,0,0,0); 
    let port = 8080;
    let mut buf = [0 as u8; 100];
    
// receive info from driverstations
//let mut dsServer =  UdpSocket::bind((ip,port)).unwrap();
    //dsServer.set_read_timeout(None);
    //let mut ts = TcpStream::new((ip,port));
    let tcpL = TcpListener::bind((ip,port)).unwrap();
    for stream  in tcpL.incoming() { //wait for incoming from putty
        let mut conn = stream.unwrap();
        println!("accepted connection");
        loop {
            let sizeMsg = conn.read(&mut buf).unwrap(); //read a message and echo it in reverse back
            let bufRet = &mut buf[..sizeMsg];
            bufRet.reverse();
            conn.write(&bufRet);
        }
    }
}
