use std::mem::{size_of, transmute_copy, transmute};
use std::slice::from_raw_parts;

#[repr(C)]
pub struct robot_control_2015 {
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
impl robot_control_2015 {
    pub fn new() -> Self {
        let ret = robot_control_2015 {
            packetIndex: 0,
            unknown: 0,
            curMode: Modes::Dtest,
            state: 0,
            voltage_greater: 0,
            voltage_lesser: 0,
        };
        ret
    }

    pub fn generate(ctrl: &mut Self ) -> Self {
        let mut new_packet = Self::new();
        ctrl.packetIndex += 1;
        new_packet.packetIndex = ctrl.packetIndex;
        new_packet.voltage_greater = 12;
        new_packet.state = 0x30; //ask shawn about this?
        new_packet.curMode = Modes::Dtest; //set to

        new_packet //return
    }

    /// this is potentially the most dangerous
    /// code in the entire code base.
    /// it relies  on serveral things
    /// endianess, packed struct, intialization.
    /// layout info.
    /// this is needed because I have to serialize my packets
    /// into the correct format for me to be able to read them.
    pub fn write_to_buf(&self, buf: &mut [u8]) {
        let raw_ptr: *const Self = self as *const Self;
        // get bytes representation of stuct
        let bytes = unsafe { from_raw_parts(raw_ptr as *mut u8, size_of::<Self>()) };
        buf.copy_from_slice(bytes);
    }
}
