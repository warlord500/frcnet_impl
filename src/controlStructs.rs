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

struct commonControlData2015 {
     packetIndex : u8,
     unknown : u8,
     state : u8,
     station : u8,
     joysticks : [joystickData; 3]

}

struct RobotControl2015 {
    packetIndex : u8,
    unknown : u8,
    mode : u8,
    state : u8,
    voltage_greater : u8,
    voltage_lesser : u8
}
