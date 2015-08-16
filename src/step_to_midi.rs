// Generate midi from step data
// Each step is a list of notes to trigger, repeated notes are retriggered

use midi_variable_len;

const MIDI_SUBDIVISIONS: usize = 96;
const STEP_TICKS: usize = MIDI_SUBDIVISIONS / 4;

const MIDI_FILE_HEADER: [u8; 14] = [
    'M' as u8,                          // tag
    'T' as u8,
    'h' as u8,
    'd' as u8,
    0, 0, 0, 6,                         // lengh
    0, 0,                               // format
    0, 1,                               // num tracks
    (MIDI_SUBDIVISIONS >> 8) as u8,     // divisions
    (MIDI_SUBDIVISIONS & 0xff) as u8
];

const CHANNEL: u8 = 0;
const VELOCITY: u8 = 100;

struct MidiDataIter<Steps: Iterator<Item=Vec<u8>>> {
    step_iter: Steps,
    blank_steps: u8
}

fn map_steps_to_track_data<'a, Steps: Iterator<Item=Vec<u8>> + 'a>(step_iter: Steps)
    -> Box<Iterator<Item=u8> + 'a> {

    // one giant scan/flat map oparation
    Box::new(
        step_iter.scan(0, |blank_steps, notes| {
            let mut midi_data = Vec::<u8>::new();

            if !notes.is_empty() {
                for (i, note) in notes.iter().enumerate() {
                    // differential timestamp - after the first message use 0 for simultaneous
                    midi_data.extend(midi_variable_len::enc(if i == 0 { *blank_steps * STEP_TICKS } else { 0 }));
                    // note on message
                    midi_data.push(0x90 | CHANNEL);
                    midi_data.push(*note);
                    midi_data.push(VELOCITY);
                }
                for (i, note) in notes.iter().enumerate() {
                    // first message is one step after previous, other messages are simultaneous
                    midi_data.extend(midi_variable_len::enc(if i == 0 { STEP_TICKS } else { 0 }));

                    // note off message
                    midi_data.push(0x80 | CHANNEL);
                    midi_data.push(*note);
                    midi_data.push(VELOCITY);
                }
                *blank_steps = 0;
            }
            else {
                *blank_steps += 1;
            }

            Some(midi_data)
        })
        .flat_map(|midi_vec| midi_vec)
    )
}
