use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;
use vvf::version;

#[derive(Parser)]
#[command(name = "vectovid")]
#[command(about = "VVF desktop - pack and play SVG-based videos", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack SVG frames into a .vvf file
    Pack {
        /// Directory containing SVG frames
        frames_dir: PathBuf,
        /// Output .vvf path
        output: PathBuf,
        /// Frames per second
        #[arg(short, long)]
        fps: Option<u32>,
        /// Optional audio file to include
        #[arg(short, long)]
        audio: Option<PathBuf>,
    },

    /// Print metadata about a .vvf file (and optionally extract)
    Play {
        /// Input .vvf file
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("vectovid desktop - VVF player and packer");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Using vvf core library version: {}", version());

    match cli.command {
        Commands::Pack { frames_dir, output, fps, audio } => {
            let fps = fps.unwrap_or(12);
            // Read SVG files
            let mut entries: Vec<_> = fs::read_dir(&frames_dir)?
                .filter_map(Result::ok)
                .filter(|e| {
                    if let Some(ext) = e.path().extension() {
                        ext == "svg"
                    } else {
                        false
                    }
                })
                .collect();
            // Sort by file name
            entries.sort_by_key(|e| e.file_name());

            let mut frames = Vec::new();
            for ent in entries {
                let txt = fs::read_to_string(ent.path())?;
                frames.push(txt);
            }

            let audio_bytes = if let Some(p) = audio {
                Some(fs::read(p)?)
            } else {
                None
            };

            let bytes = vvf::pack_vvf_native(fps, frames, audio_bytes)
                .map_err(|e| anyhow::anyhow!(e))?;

            fs::write(&output, &bytes)?;
            println!("Wrote {}", output.display());
        }

        Commands::Play { input } => {
            // Open zip and read meta.json
            let data = fs::read(&input)?;
            let reader = std::io::Cursor::new(&data);
            let mut zip = zip::ZipArchive::new(reader)?;
            if let Ok(mut f) = zip.by_name("meta.json") {
                let mut s = String::new();
                use std::io::Read;
                f.read_to_string(&mut s)?;
                println!("meta.json:\n{}", s);
            } else {
                println!("meta.json not found in {}", input.display());
            }

            // List frame files
            let mut count = 0;
            for i in 0..zip.len() {
                let file = zip.by_index(i)?;
                let name = file.name().to_string();
                if name.starts_with("frames/") {
                    count += 1;
                }
            }
            println!("Found {} frames", count);
        }
    }

    Ok(())
}
