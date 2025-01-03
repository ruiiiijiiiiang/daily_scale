# Daily Scale

Welcome to Daily Scale! This is a simple program that will help you practice a new scale every day.

## Basic Usage

When executed without any options, the program will randomly select a root note, a scale and a starting fret. (The randomness is seeded by the current date so you have all day to work on it.) Here's an example of the output:

    |---A#---|--------|---C----|--------|---D---|
    |---F----|--------|---G----|--------|---A---|
    |--------|---D----|---D#---|--------|---F---|
    |========|===A====|===A#===|========|===C===|
    |===D#===|========|===F====|========|===G===|
    |===A#===|========|===C====|========|===D===|
    |   6    |   7    |   8    |   9    |   10  |
    Here's the scale of the day: D Phrygian starting at fret 6 in Standard E (6 string) tuning
    The notes in this scale are: D, D#, F, G, A, A#, C

## Features

        Usage: daily-scale [OPTIONS]

        Options:
        -t, --tuning <TUNING>
                Your choice of tuning [default: standard-e6] [possible values: standard-e6, open-g6, open-e6, open-d6, open-c6, open-a6, drop-d6, standard-d6, drop-c-sharp6, standard-c-sharp6, drop-c6, standard-c6, standard-b7, drop-a7, standard-a7, all-fourths7]
        -n, --root-notes <ROOT_NOTES>
                Pool of root notes of the scale [possible values: a-flat, a, a-sharp, b-flat, b, c, c-sharp, d-flat, d, d-sharp, e-flat, e, f, f-sharp, g-flat, g, g-sharp]
        -s, --scales <SCALES>
                Pool of scales [possible values: major, harmonic-minor, melodic-minor, natural-minor, pentatonic-major, pentatonic-minor, pentatonic-blues, pentatonic-neutral, whole-diminished, half-diminished, ionian, dorian, phrygian, lydian, mixolydian, aeolian, locrian]
        -f, --starting-frets <STARTING_FRETS>
                Pool of frets to start the scale on
        -r, --full-randomness
                If true, the rng will no longer use today's date as seed
        -h, --help
                Print help
        -V, --version
                Print version

## Available Options

### Tunings

- Standard E (6 string)
- Open G (6 string)
- Open E (6 string)
- Open D (6 string)
- Open C (6 string)
- Open A (6 string)
- Drop D (6 string)
- Standard D (6 string)
- Drop C# (6 string)
- Standard C# (6 string)
- Drop C (6 string)
- Standard C (6 string)
- Standard B (7 string)
- Drop A (7 string)
- Standard A (7 string)
- All fourths (7 string)

### Scales

- Major
- Harmonic Minor
- Melodic Minor
- Natural Minor
- Pentatonic Major
- Pentatonic Minor
- Pentatonic Blues
- Pentatonic Neutral
- Whole Diminished
- Half Diminished
- Ionian
- Dorian
- Phrygian
- Lydian
- Mixolydian
- Aeolian
- Locrian

