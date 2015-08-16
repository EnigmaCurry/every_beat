// Each instrument has a 16 note pattern
struct InstrumentPattern([bool; 16]);

// For simplicity, assume the drum machine has 4 instruments (kick, snare, closed hat, open hat)
struct MachinePattern([InstrumentPattern; 4]);

fn main() {
    println!("Hello, world!");
}

impl InstrumentPattern {
    fn other_from_u16(num: u16) -> Self {
        // This could be a straightforward mapping of bit positions to pattern positions
        // However, to generate more common patterns first, I'm going to map the lowest bits to the
        // 1/4s, then the 1/8s between them, then finally the raminaing 1/16ths
        InstrumentPattern ([
            ((num & 0x0001) != 0),  // 1
            ((num & 0x0100) != 0),
            ((num & 0x0010) != 0),  // and
            ((num & 0x0200) != 0),
            ((num & 0x0002) != 0),  // 2
            ((num & 0x0400) != 0),
            ((num & 0x0020) != 0),  // and
            ((num & 0x0800) != 0),
            ((num & 0x0004) != 0),  // 3
            ((num & 0x1000) != 0),
            ((num & 0x0040) != 0),  // and
            ((num & 0x2000) != 0),
            ((num & 0x0008) != 0),  // 4
            ((num & 0x4000) != 0),
            ((num & 0x0080) != 0),  // and
            ((num & 0x8000) != 0),
        ])
    }

    fn kick_from_u16(num: u16) -> Self {
        // the kick gets its own mappging so that low numbered patterns get a kick at the start of
        // the bar
        InstrumentPattern ([
            ((num & 0x8000) == 0),  // 1
            ((num & 0x0010) != 0),
            ((num & 0x0001) != 0),  // and
            ((num & 0x0020) != 0),
            ((num & 0x2000) == 0),  // 2
            ((num & 0x0040) != 0),
            ((num & 0x0002) != 0),  // and
            ((num & 0x0080) != 0),
            ((num & 0x4000) == 0),  // 3
            ((num & 0x0100) != 0),
            ((num & 0x0004) != 0),  // and
            ((num & 0x0200) != 0),
            ((num & 0x1000) != 0),  // 4
            ((num & 0x0400) != 0),
            ((num & 0x0008) != 0),  // and
            ((num & 0x0800) != 0),
        ])
    }
}

impl MachinePattern {
    fn from_u64(num: u64) -> Self {
        MachinePattern ([
            InstrumentPattern::kick_from_u16(  (num & 0xffff)                    as u16),
            InstrumentPattern::other_from_u16(((num & 0xffff0000)         >> 16) as u16),
            InstrumentPattern::other_from_u16(((num & 0xffff00000000)     >> 32) as u16),
            InstrumentPattern::other_from_u16(((num & 0xffff000000000000) >> 48) as u16),
        ])
    }
}
