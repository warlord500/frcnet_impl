use std::mem::{size_of, transmute_copy, transmute};
use std::slice::from_raw_parts;

#[repr(C)]
pub struct robotControl2015 {
    packetIndex: u8,
    unknown: u8,
    curMode: Modes,
    state: u8,
    voltage_greater: u8,
    voltage_lesser: u8,
}
#[repr(C)]
enum Modes {
    Dtele,
    Dtest,
    DAuto,
    Etele,
    ETest,
    EAuto,
}
impl robotControl2015 {
    pub fn new() -> Self {
     let ret = robotControl2015 {
        packetIndex: 0,
        unknown: 0,
        curMode: Modes::Dtest,
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
           newPacket.curMode = Modes::Dtest; //set to 
           
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
