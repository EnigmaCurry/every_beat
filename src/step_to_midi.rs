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
    notes_active: Vec<u8>,
    blank_steps: u8
}

impl<Steps: Iterator<Item=Vec<u8>>> MidiDataIter<Steps> {
    fn new(step_iter: Steps) -> Self {
        MidiDataIter::<Steps> {
            step_iter: step_iter,
            notes_active: Vec::new(),
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
        if this_step == None && self.notes_active.is_empty() {
            None
        }
        else {
            let mut midi_data = Vec::<u8>::new();
            // finish previous notes
            for (i, note) in self.notes_active.iter().enumerate() {
                // first message is one step after previous, other messages are simultaneous
                midi_data.push(if i == 0 { STEP_TICKS } else { 0 });

                // note off message
                midi_data.push(0x80 | CHANNEL);
                midi_data.push(*note);
                midi_data.push(VELOCITY);
            }
            self.notes_active.clear();

            if let Some(notes) = this_step {
                if !notes.is_empty() {
                    for note in notes.iter() {
                        // timestamp
                        midi_data.push(self.blank_steps * STEP_TICKS);
                        // note on message
                        midi_data.push(0x90 | CHANNEL);
                        midi_data.push(*note);
                        midi_data.push(VELOCITY);
                    }
                    self.notes_active = notes;
                    self.blank_steps = 0;
                }
                else {
                    self.blank_steps += 1;
                }
            }

            Some(midi_data)
        }
    }
}
