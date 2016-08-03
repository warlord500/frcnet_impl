use std::mem::size_of;
use std::slice::from_raw_parts;
/// this the receive packet format
#[repr(C)]
pub struct commonControlData2015 {
    packetIndex: u8,
    unknown: u8,
    state: u8,
    station: u8,
    joysticks: [implt::joystickData; 4],
}
impl commonControlData2015 {
    pub fn blankPack() -> Self {
      let ret =  commonControlData2015 {
            packetIndex: 0,
            unknown: 0,
            state: 0,
            station: 0,
            joysticks: [implt::joystickData::new(); 4],
        };
      ret

    }
}

// send packet format
#[repr(C)]
pub struct robotControl2015 {
    packetIndex: u8,
    unknown: u8,
    curMode: implt::Modes,
    state: u8,
    voltage_greater: u8,
    voltage_lesser: u8,
}
impl robotControl2015 {
    pub fn new() -> Self {
     let ret = robotControl2015 {
        packetIndex: 0,
        unknown: 0,
        curMode: implt::Modes::Dtest,
        state: 0,
        voltage_greater: 0,
        voltage_lesser: 0,
        };
        ret
    }

    /// this is potentially the most dangerous
    /// code in the entire code base.
    /// it relies  on serveral things
    /// endianess, packed struct, intialization.
    /// layout info.
    /// this is needed because I have to serialize my packets
    /// into the correct format for me to be able to read them.
    pub fn generate(ctrl : &mut Self) -> Self {
       let mut newPacket = Self::new();
           ctrl.packetIndex += 1;
           newPacket.packetIndex = ctrl.packetIndex;
           newPacket.voltage_greater = 12;
           newPacket.state = 0x30; //ask shawn about this?
           newPacket.curMode = implt::Modes::Dtest; //set to 
           
           newPacket //return
   }
    pub fn write_to_buf(&self, buf: &mut [u8]) {
        let rawPtr: *const Self = self as *const Self;
        // get bytes representation of stuct
        let bytes = 
            unsafe {from_raw_parts(rawPtr as *mut u8, 
                                   size_of::<Self>()) };
        buf.copy_from_slice(bytes);
    }
}
#[repr(C)]
enum EmbeddeedDynamicChunk {
    UsersDataHigh,
    Errors,
    DataLow,
    Count,
}
mod implt {
    // private details of the module
    // dont reorder
    #[repr(C)]
    pub enum Modes {
        Dtele,
        Dtest,
        DAuto,
        Etele,
        ETest,
        EAuto,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct joystickData {
        joystick_size: u8,
        pov_up_to_size: u8,
        axes_size: u8,
        axis: [u8; 6],
        unknown: u8,
        buttons: u16,
        pov_size: u8, // unsure
        pov: [u8; 2],
    }
    impl joystickData {
        pub fn new() -> Self {
            joystickData {
                joystick_size: 0,
                pov_up_to_size: 0,
                axes_size: 0,
                axis: [0; 6],
                unknown: 0,
                buttons: 0,
                pov: [0; 2],
                pov_size:0,
            }
        }
    }
    impl Copy for joystickData{}
}
