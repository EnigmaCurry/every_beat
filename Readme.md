# Every beat

Drawing inspiration from Autechre who once made a track of entirely
non-repeating beats, I've written a generator for every possible 1 bar drum
machine pattern using a 16 step sequencer and a restricted set of drums (kick,
snare, hats).

# Usage

Run from the command line and specify an output file. You can use the options
--start, --step and --bars to specify which of the possible patterns you want
and how many bars to generate. The output files are midi.

# The process

With the restrictions, generating every beat is actually a really straight
forward task. Each instrument as 16 steps which can be on or off, this can
be directly mapped to a 16 bit value. Having 4 instruments means that the
overall sequence is 64 bits. We can start from 0 and count them!

This simplicity has a problem; a direct mapping from integers to sequences makes
it hard to find musical sounding patterns in the 2^64 possibilities.
Fortunately, ANY mapping that uses every bit of the input value exactly one will
still contain all of the 2^64 possible beats. This means that the mapping can be
chosen for musicality. An easy technique to achieve this is to map pattern
zero to something good, I've used the Amen break.

# Conversion to midi

The pattern module generates a number of steps with each step represented as a
list of notes to trigger. The midi output module contains exactly enough
of the midi format to turn these into actual midi notes. It steps through and
outputs "note on" messages followed immediately by "note off" messages. This is
fairly straightforward with the only issue being some slight fussing over
differential timestamps.
