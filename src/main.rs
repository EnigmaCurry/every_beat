mod note;
mod step_to_midi;
mod midi_variable_len;
mod patterns;

use note::Note;

extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs::File;
use std::io::Write;


fn u64_validator(value: String) -> Result<(), String> {
    value.parse::<u64>().map(|_| () ).map_err(|_| "not a number".to_owned() )
}

fn main() {
    let args = App::new("myapp")
        .version("0.1")
        .author("monsieursquirrel")
        .about("Generates every possible beat (for a very crappy drum machine)")
        .arg(Arg::with_name("START")
            .help("Sets the pattern number to start from")
            .required(false)
            .takes_value(true)
            .validator(u64_validator)
            .long("start"))
        .arg(Arg::with_name("STEP")
            .help("Sets the pattern number change between each bar")
            .required(false)
            .takes_value(true)
            .validator(u64_validator)
            .long("step"))
        .arg(Arg::with_name("NUM_BARS")
            .help("Sets the number of bars to generate")
            .required(false)
            .takes_value(true)
            .validator(u64_validator)
            .long("bars"))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use")
            .required(true)
            .index(1))
        .get_matches();

    let outfile = args.value_of("OUTPUT").unwrap();  // should have been rejected if blank
    let start: u64 = args.value_of("START").and_then(|arg_str| arg_str.parse().ok() ).unwrap_or(0);
    let step: u64 = args.value_of("STEP").and_then(|arg_str| arg_str.parse().ok() ).unwrap_or(1);
    let num_bars: u64 = args.value_of("NUM_BARS").and_then(|arg_str| arg_str.parse().ok() ).unwrap_or(128);

    // build a big note sequence
    let mut note_sequence = Vec::<Vec<Note>>::with_capacity(num_bars as usize * 16);
    for i in (0..num_bars) {
        let pattern_num = i.wrapping_mul(step).wrapping_add(start);
        let current_pattern = patterns::MachinePattern::from_u64(pattern_num);
        note_sequence.extend(current_pattern.step_iterator());
    }

    // write it out to a midi file
    let file_data = step_to_midi::midi_file(note_sequence.into_iter());
    File::create(outfile)
    .and_then(|mut file| file.write_all(&file_data[..]) );
}
