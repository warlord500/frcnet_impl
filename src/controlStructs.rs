///this the receive packet format
#[repr(C)]
pub struct commonControlData2015 {
     packetIndex : u8,
     unknown : u8,
     state : u8,
     station : u8,
     joysticks : [joystickData; 4],
    dynamicSize : u8
}
impl ControlData2015 {
    pub fn generatePacket() -> self{
        

    }
    pub fn blankPack() -> self {}
}

// send packet format
#[repr(C)]
struct RobotControl2015 {
    packetIndex : u8,
    unknown : u8,
    curMode : Modes,
    state : u8,
    voltage_greater : u8,
    voltage_lesser : u8
}

#[repr(C)]
enum  EmbeddeedDynamicChunk {
    UsersDataHigh,
    Errors,
    DataLow,
    Count
}
mod implt {
    //dont reorder 
    #[repr(C)]
    enum Modes{ 
        Dtele, 
        Dtest,
        DAuto,
        Etele,
        ETest,
        EAuto
    }

    #[repr(C)]
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

