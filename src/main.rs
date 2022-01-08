use clap::Parser;
use env_logger::Env;
use lazy_static::lazy_static;
use log::trace;
use musical_scales::Pitch;
use regex::Regex;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::fs::{self, DirEntry};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// A directory containing WAV files with names containing note names (e.g. dubstab-C1.wav or
    /// harpischord-A#3.WAV).
    #[clap(short, long, default_value = ".")]
    input_directory: String,
    /// Wether to set the samples to loop (`-loop=1` or `-loop=0`)
    #[clap(short, long)]
    loop_sample: bool,
    /// Offset in semitone to add to the note written in the file names.
    #[clap(short, long, default_value_t = -24)]
    offset: i16,
    #[clap(short, long)]
    verbose: bool,
}

fn dir2pitch(entry: &DirEntry) -> Pitch {
    lazy_static! {
        // allow having files named e.g. C#1.wav and C1#.wav.
        static ref RE: Regex = Regex::new(r"([a-gA-G]\d[#♯♮b♭]?|[a-gA-G][#♯♮b♭]?\d)").unwrap();
    }
    // find pitch notation in wav files
    let filename = entry.file_name();
    let string = filename.to_str().unwrap();
    for cap in RE.captures_iter(string) {
        return match Pitch::try_from(&cap[1]) {
            Ok(p) => p,
            Err(_) => Pitch::try_from("C-1").unwrap(),
        };
    }
    Pitch::try_from("C-1").unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    if args.verbose {
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    } else {
        env_logger::Builder::from_env(Env::default()).init();
    }

    let directory = args.input_directory;

    let wav_dir = Path::new(&directory);
    let wav_dir_entries = fs::read_dir(wav_dir)?;

    // find all WAV file in `directory`
    let mut entries = wav_dir_entries
        .filter_map(|e| {
            e.ok().and_then(|e| {
                let p = e.path();
                let ext = p.extension();
                if ext == Some(OsStr::new("wav")) || ext == Some(OsStr::new("WAV")) {
                    return Some(e);
                } else {
                    return None;
                }
            })
        })
        .collect::<Vec<_>>();

    // sort by midi note number
    entries.sort_by(|lhs, rhs| dir2pitch(lhs).to_midi().cmp(&dir2pitch(rhs).to_midi()));

    let mut content = String::from("disting playlist v1\n");
    let loop_integer = if args.loop_sample { 1 } else { 0 };
    content.push_str(&format!("- loop={}\n", loop_integer));

    let count = entries.len();
    for index in 0..count {
        let p = entries[index].path();
        let file_name = p.file_name();
        let name_str = file_name.unwrap().to_str().unwrap();
        content.push_str(&format!("{}\n", name_str));
        let pitch = dir2pitch(&entries[index]);
        let midi_note = pitch.to_midi() as i16;
        trace!("{} is pitch {:?}", name_str, pitch);
        content.push_str(&format!("- natural {}\n", midi_note - args.offset));
        if index > 0 {
            let prev_note = dir2pitch(&entries[index - 1]).to_midi() as i16;
            let switch_value = prev_note + (midi_note - prev_note) / 2;
            if switch_value != 0 {
                content.push_str(&format!("- switch {}\n", switch_value - args.offset));
            }
        }
    }

    File::create(wav_dir.join(Path::new("playlist.txt")))?.write_all(&content.as_bytes())?;

    Ok(())
}
