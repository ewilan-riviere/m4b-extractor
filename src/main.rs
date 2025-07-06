mod args;
mod commands;
mod metadata;
mod utils;

use anyhow::Result;
use args::Args;
use clap::Parser;
use std::path::Path;

fn main() -> Result<()> {
    let args = Args::parse();

    // Determine output directory, fallback to default naming
    let output_dir = match &args.output {
        Some(dir) => dir.clone(),
        None => {
            let input_path = std::path::Path::new(&args.input);
            let input_stem = input_path.file_stem().unwrap().to_string_lossy();
            format!("{}_chapters", input_stem)
        }
    };

    println!("Input: {}", args.input);
    println!("Output directory: {}", output_dir);
    println!("Keep original m4b files (no convert): {}", args.no_convert);
    println!("Quality (only use for conversion): {}", args.quality);
    println!("Sanitize filenames: {}", args.sanitize);
    println!("");

    utils::check_commands(&["ffmpeg", "ffprobe", "jq", "yq"])?;

    let input_path = Path::new(&args.input);
    if !input_path.exists() {
        anyhow::bail!("Input file '{}' does not exist", args.input);
    }

    if std::path::Path::new(&output_dir).exists() {
        println!(
            "⚠️ Output directory '{}' exists, deleting it...",
            output_dir
        );
        std::fs::remove_dir_all(&output_dir)?;
    }
    std::fs::create_dir_all(&output_dir)?;

    let metadata_json = metadata::run_ffprobe_json(&args.input)?;

    println!("📂 Splitting chapters into: {}", &output_dir);
    commands::split_chapters(&args, &output_dir, &metadata_json)?;

    println!("💾 Exporting metadata files...");
    metadata::write_metadata_files(&output_dir, &metadata_json)?;

    metadata::extract_cover(&args.input, &output_dir, &metadata_json)?;

    if !args.no_convert {
        commands::convert_to_mp3(&output_dir, args.quality)?;
    } else {
        println!("⚠️ MP3 conversion disabled, keeping .m4b files.");
    }

    println!("\n✅ Done! All files saved to: {}", &output_dir);

    Ok(())
}
