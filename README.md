# `disting-wav-2-playlist`

Create a file `playlist.txt` for an Expert Sleeper Disting Ex or Disting module,
for sample playback, from a directory containing WAV files with the note name in
the file name.

# Usage

From a directory that contains the following files:

```
dubstab-A#0.wav
dubstab-A3.wav
dubstab-B4.wav
dubstab-C2.wav
dubstab-D3.wav
dubstab-E4.wav
dubstab-F1.wav
dubstab-G2.wav
```

this program will create a file named `playlist.txt` containing the following
text:

```
disting playlist v1
- loop=0
dubstab-A#0.wav
- natural 46
dubstab-F1.wav
- natural 53
- switch 49
dubstab-C2.wav
- natural 60
- switch 56
dubstab-G2.wav
- natural 67
- switch 63
dubstab-D3.wav
- natural 74
- switch 70
dubstab-A3.wav
- natural 81
- switch 77
dubstab-E4.wav
- natural 88
- switch 84
dubstab-B4.wav
- natural 95
- switch 91
```

# Command-line options

```
disting-wav-2-playlist 0.1.0

USAGE:
    disting-wav-2-playlist [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -i, --input-directory <INPUT_DIRECTORY>
            A directory containing WAV files with names containing note names (e.g. dubstab-C1.wav
            or harpischord-A#3.WAV) [default: .]

    -l, --loop-sample
            Wether to set the samples to loop (`-loop=1` or `-loop=0`)

    -o, --offset <OFFSET>
            Offset in semitone to add to the note written in the file names [default: -24]

    -v, --verbose

    -V, --version
            Print version information
```

