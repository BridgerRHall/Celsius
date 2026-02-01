// 4FF00
// bit 0: A or Right -> 0 is pressed
// bit 1  B or Left -> 0 is pressed
// bit 2  Select or Up -> 0 is pressed
// bit 3  Start or Down -> 0 is pressed
// bit 4  Select D-pad -> 0 is D-pad active
// bit 4  Select Buttons -> 0 is Buttons active
// bit 6 unused
// bit 7 unused

// joypad intterup at $0060 whenever a button bit transitions for 1 to 0


pub joypad_state: u8;

