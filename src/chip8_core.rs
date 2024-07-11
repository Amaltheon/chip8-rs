pub struct State {
    pub ram: [u8; 4096],
    pub screen: [[bool; 64]; 32],
    pub program_counter: u16,
    pub index: u16,
    pub stack: [u16; 64],
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v_buffer: [u8; 16],
    pub screen_width: u32,
    pub screen_height: u32
}