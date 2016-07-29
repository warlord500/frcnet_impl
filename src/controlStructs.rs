#[repr(c)]
struct joystickData {
  joystick_size : u8,
  pov_up_to_size : u8,
  axes_size : u8,
  axis : [u8; 6], 
  unkown : u8,
  button : u8,
  pov_size : u8,
  pov : [u8; 2]
}

#[repr(c)]
struct commonControlData2015 {
     packetIndex : u8,
     unknown : u8,
     state : u8,
     station : u8,
     joysticks : [joystickData; 3]
}

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
#[repr(c)]
enum Modes{ // this cannot be REORDERED  driverStation uses 0 -6 in order for these modes
    Dtele,
    Dtest,
    DAuto,
    Etele,
    ETest,
    EAuto
}
