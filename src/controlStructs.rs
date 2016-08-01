///this the receive packet format
#[repr(c)]
pub struct commonControlData2015 {
     packetIndex : u8,
     unknown : u8,
     state : u8,
     station : u8,
     joysticks : [joystickData; 4],
    dynamicSize : u8
}
pub struct dynaChunk {
    id : u8,
    data : vec<u8>,
}
impl ControlData2015 {
    pub fn generatePacket(){


    }
}

// send packet format
#[repr(c)]
struct RobotControl2015 {
    packetIndex : u8,
    unknown : u8,
    curMode : Modes,
    state : u8,
    voltage_greater : u8,
    voltage_lesser : u8
}

#[repr(c)]
enum  EmbeddeedDynamicChunk {
    UsersDataHigh,
    Errors,
    DataLow,
    Count
}
mod implt {
    //dont reorder 
    #[repr(c)]
    enum Modes{ 
        Dtele, 
        Dtest,
        DAuto,
        Etele,
        ETest,
        EAuto
    }

    #[repr(c)]
    struct joystickData {
      joystick_size : u8,
      pov_up_to_size : u8,
      axes_size : u8,
      axis : [u8; 6], 
      unkown : u8,
      buttons : u16,
      pov_size : u8, //unsure
      pov : [u8; 2]
    }

}

