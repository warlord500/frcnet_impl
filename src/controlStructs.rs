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
        let ret = commonControlData2015 {
            packetIndex: 0,
            unknown: 0,
            state: 0,
            station: 0,
            joysticks: [implt::joystickData::new(); 4],
        };
        ret

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
                pov_size: 0,
            }
        }
    }
    impl Copy for joystickData {}
}
