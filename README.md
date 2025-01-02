# Daily Scale

Welcome to Daily Scale! This is a simple program that will help you practice a new scale every day.

## Simple Usage

When executed without any options, the program will randomly select a root note, a scale and a starting fret. (The randomness is seeded by the current date so you have all day to work on it.) Here's an example of the output:

    |---A#---|--------|---C----|--------|---D---|
    |---F----|--------|---G----|--------|---A---|
    |--------|---D----|---D#---|--------|---F---|
    |========|===A====|===A#===|========|===C===|
    |===D#===|========|===F====|========|===G===|
    |===A#===|========|===C====|========|===D===|
    |   6    |   7    |   8    |   9    |   10  |
    Here's the scale of the day: D Phrygian starting at fret 6 in Standard E tuning
    The notes in this scale are: D, D#, F, G, A, A#, C

## All Options

    Usage: daily-scale [OPTIONS]

    Options:
      -t, --tuning <TUNING>
              Your choice of tuning [default: standard-e6] [possible values: standard-e6, drop-d6, standard-b7, drop-a7]
      -n, --root-notes <ROOT_NOTES>
              Pool of root notes to randomly choose from [possible values: a, a-sharp, b, c, c-sharp, d, d-sharp, e, f, f-sharp, g, g-sharp]
      -s, --scales <SCALES>
              Pool of scales to randomly choose from [possible values: major, harmonic-minor, melodic-minor, natural-minor, pentatonic-major, pentatonic-minor, pentatonic-blues, pentatonic-neutral, whole-diminished, half-diminished, ionian, dorian, phrygian, lydian, mixolydian, aeolian, locrian]
      -f, --starting-frets <STARTING_FRETS>
              Your choice of frets to start the scale on
      -h, --help
              Print help
      -V, --version
              Print version

For example, to play in drop D, randomly choose a root note from [A, C, D#], a scale from [Major, Harmonic Minor], and a starting fret from [5, 7, 12], you can run:

    daily-scale -n drop-d6 a,c,d-sharp -s major,harmonic-minor -f 5,7,12

Now set it to your shell's greeting message and start practicing!
