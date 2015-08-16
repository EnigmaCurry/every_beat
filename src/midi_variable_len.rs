use std::mem;

pub fn enc(val: usize) -> Vec<u8> {
    // Count how many bytes are needed
    // can only store 7 bits per byte in this scheme

    // start at 1 since at least 1 byte must be output
    let mut count = 1;
    while val >> (count * 7) != 0 {
        count += 1;
    }

    (0..count)
    .rev()
    .map(|i| ((val >> (i * 7)) & 0x7f) as u8 | if (i == 0) { 0 } else { 0x80 } )
    .collect()
}

#[test]
fn sample_values() {
    assert!(enc(127) == [0x7f]);
    assert!(enc(255) == [0x81, 0x7f]);
    assert!(enc(32768) == [0x82, 0x80, 0x00]);
}
