mod step_to_midi;
mod midi_variable_len;

use std::fs::File;
use std::io::Write;

// Each instrument has a 16 note pattern
struct InstrumentPattern([bool; 16]);

// For simplicity, assume the drum machine has 4 instruments (kick, snare, closed hat, open hat)
struct MachinePattern([InstrumentPattern; 4]);

// drum note numbers
const KICK_NOTE: u8 = 36;
const SNARE_NOTE: u8 = 38;
const CH_NOTE: u8 = 42;
const OH_NOTE: u8 = 46;

fn main() {
    println!("Hello, world!");

    let outfile = "out.mid";
    let start = 0x1010101010101010;
    let step = 153;
    let num_bars: u64 = 20;

    // build a big note sequence
    let mut note_sequence = Vec::<Vec<u8>>::with_capacity(num_bars as usize * 16);
    for i in (0..num_bars) {
        let pattern_num = i.wrapping_mul(step).wrapping_add(start);
        let current_pattern = MachinePattern::from_u64(pattern_num);
        note_sequence.extend(current_pattern.step_iterator());
    }

    // write it out to a midi file
    let file_data = step_to_midi::midi_file(note_sequence.into_iter());
    File::create(outfile)
    .and_then(|mut file| file.write_all(&file_data[..]) );
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

    fn step_iterator<'a>(&'a self) -> Box<Iterator<Item=Vec<u8>> + 'a> {
        Box::new(
            // zip the four patterns together to get an iterator giveing
            // the status of the 4 notes at each step
            self.0[0].0.iter()
            .zip(self.0[1].0.iter())
            .zip(self.0[2].0.iter())
            .zip(self.0[3].0.iter())
            .map(|(((a, b), c), d)| (a, b, c, d))
            // map the four note statuses to midi note values
            .map(|(a, b, c, d)| {
                let mut notes = Vec::with_capacity(4);
                if *a { notes.push(KICK_NOTE) }
                if *b { notes.push(SNARE_NOTE) }
                if *c { notes.push(CH_NOTE) }
                if *d { notes.push(OH_NOTE) }
                notes
            })
        )
    }
}
