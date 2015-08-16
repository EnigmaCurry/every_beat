// Generate midi from step data
// Each step is a list of notes to trigger, repeated notes are retriggered

const MIDI_SUBDIVISIONS: u8 = 12;
const STEP_TICKS: u8 = MIDI_SUBDIVISIONS / 4;

const MIDI_FILE_HEADER: [u8; 14] = [
    'M' as u8,                  // tag
    'T' as u8,
    'h' as u8,
    'd' as u8,
    0, 0, 0, 6,                 // lengh
    0, 0,                       // format
    0, 1,                       // num tracks
    0, MIDI_SUBDIVISIONS        // divisions
];

const CHANNEL: u8 = 0;
const VELOCITY: u8 = 100;

struct MidiDataIter<Steps: Iterator<Item=Vec<u8>>> {
    step_iter: Steps,
    blank_steps: u8
}

impl<Steps: Iterator<Item=Vec<u8>>> MidiDataIter<Steps> {
    fn new(step_iter: Steps) -> Self {
        MidiDataIter::<Steps> {
            step_iter: step_iter,
            blank_steps: 0,
        }
    }
}

// each step produces a Vec of data bytes that can be flatmapped into a midi file
impl<Steps: Iterator<Item=Vec<u8>>> Iterator for MidiDataIter<Steps> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let this_step = self.step_iter.next();

        // end iteration if there's no new notes and no note off events to output

        if let Some(notes) = this_step {
            let mut midi_data = Vec::<u8>::new();

            if !notes.is_empty() {
                for (i, note) in notes.iter().enumerate() {
                    // differential timestamp - after the first message use 0 for simultaneous
                    midi_data.push(if i == 0 { self.blank_steps * STEP_TICKS } else { 0 });
                    // note on message
                    midi_data.push(0x90 | CHANNEL);
                    midi_data.push(*note);
                    midi_data.push(VELOCITY);
                }
                for (i, note) in notes.iter().enumerate() {
                    // first message is one step after previous, other messages are simultaneous
                    midi_data.push(if i == 0 { STEP_TICKS } else { 0 });

                    // note off message
                    midi_data.push(0x80 | CHANNEL);
                    midi_data.push(*note);
                    midi_data.push(VELOCITY);
                }
                self.blank_steps = 0;
            }
            else {
                self.blank_steps += 1;
            }

            Some(midi_data)
        }
        else {
            None
        }

    }
}
