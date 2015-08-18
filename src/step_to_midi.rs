// Generate midi from step data
// Each step is a list of notes to trigger, repeated notes are retriggered

use note::Note;
use midi_variable_len;

const MIDI_SUBDIVISIONS: usize = 96;
const STEP_TICKS: usize = MIDI_SUBDIVISIONS / 4;

const CHANNEL: u8 = 0;

fn map_steps_to_track_data<'a, Steps: Iterator<Item=Vec<Note>> + 'a>(step_iter: Steps)
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
                    midi_data.push(note.pitch);
                    midi_data.push(note.velocity);
                }
                for (i, note) in notes.iter().enumerate() {
                    // first message is one step after previous, other messages are simultaneous
                    midi_data.extend(midi_variable_len::enc(if i == 0 { STEP_TICKS } else { 0 }));

                    // note off message
                    midi_data.push(0x80 | CHANNEL);
                    midi_data.push(note.pitch);
                    midi_data.push(note.velocity);
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

// notes are expressed as (note number, velocity)
pub fn midi_file<Steps: Iterator<Item=Vec<Note>>>(step_iter: Steps) -> Vec<u8> {
    let mut file_data = Vec::new();

    // The midi file header is constant
    file_data.extend([
        'M' as u8,                                  // tag
        'T' as u8,
        'h' as u8,
        'd' as u8,
        0, 0, 0, 6,                                 // lengh
        0, 0,                                       // format (0 == single track file)
        0, 1,                                       // num tracks
        ((MIDI_SUBDIVISIONS >>  8) & 0xff) as u8,   // clock divisions
        ((MIDI_SUBDIVISIONS)       & 0xff) as u8
    ].iter());

    // track header
    file_data.extend([
        'M' as u8,  // tag
        'T' as u8,
        'r' as u8,
        'k' as u8
    ].iter());
    // the track length field needs to be set to the amount of data in the track.
    // set it to 0 initially and sort it out later
    let track_len_location = file_data.len();;
    file_data.extend([
        0, 0, 0, 0, // lengh
    ].iter());
    let total_header_len = file_data.len();

    file_data.extend(map_steps_to_track_data(step_iter));

    let data_len = file_data.len() - total_header_len;

    file_data[track_len_location + 0] = ((data_len >> 24) & 0xff) as u8;
    file_data[track_len_location + 1] = ((data_len >> 16) & 0xff) as u8;
    file_data[track_len_location + 2] = ((data_len >>  8) & 0xff) as u8;
    file_data[track_len_location + 3] = ((data_len)       & 0xff) as u8;

    file_data
}
