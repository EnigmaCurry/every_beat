pub struct Note {
    pub pitch: u8,      // midi note number
    pub velocity: u8
}

impl Note {
    pub fn new(pitch: u8, velocity: u8) -> Self {
        Note {pitch: pitch, velocity: velocity}
    }
}
